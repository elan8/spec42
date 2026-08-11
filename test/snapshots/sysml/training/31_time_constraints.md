# META
~~~ini
description=SysML Training 31 (Constraints): Time Constraints
type=file
~~~
# SOURCE
~~~sysml
package 'Time Constraints' {
	private import ISQ::TemperatureValue;
	private import ISQ::DurationValue;
	private import Time::TimeInstantValue;
	private import Time::TimeOf;
	private import Time::DurationOf;
	private import SI::h;
	private import SI::s;

	attribute def MaintenanceDone;
	
	part def Vehicle {
		attribute maintenanceTime : TimeInstantValue;
		attribute maintenanceInterval : DurationValue;
		attribute maxTemperature : TemperatureValue;
	}
	
	state healthStates {
		in vehicle : Vehicle;
		
		entry; then normal;
		
		state normal;
		accept at vehicle.maintenanceTime
			then maintenance;
		
		state maintenance {
			assert constraint { TimeOf(maintenance) > vehicle.maintenanceTime }
			assert constraint { TimeOf(maintenance) - TimeOf(normal.done) < 2 [s] }
			entry assign vehicle.maintenanceTime := vehicle.maintenanceTime + vehicle.maintenanceInterval;
		}
		accept MaintenanceDone
			then normal;
		
		constraint { DurationOf(maintenance) <= 48 [h] }
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "31_time_constraints.md"
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
        (range (start 4 16) (end 4 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 21))
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwState,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,CloseAngle,Ident,Dot,Ident,CloseCurly,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,Minus,Ident,OpenParen,Ident,Dot,Ident,CloseParen,OpenAngle,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
KwEntry,KwAssign,Ident,Dot,Ident,ColonEq,Ident,Dot,Ident,Plus,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,LtEq,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Time Constraints''
    (import_decl private 'ISQ::TemperatureValue')
    (import_decl private 'ISQ::DurationValue')
    (import_decl private 'Time::TimeInstantValue')
    (import_decl private 'Time::TimeOf')
    (import_decl private 'Time::DurationOf')
    (import_decl private 'SI::h')
    (import_decl private 'SI::s')
    (attribute_def 'MaintenanceDone')
    (part_def 'Vehicle'
      (attribute_usage 'maintenanceTime' : 'TimeInstantValue')
      (attribute_usage 'maintenanceInterval' : 'DurationValue')
      (attribute_usage 'maxTemperature' : 'TemperatureValue'))
    (state_usage 'healthStates'
      (default_ref_usage in 'vehicle' : 'Vehicle')
      (entry_action)
      (source_succession
        (default_ref_usage 'normal'))
      (state_usage 'normal')
      (target_transition)
      (state_usage 'maintenance'
        (sysml_decl
          (result_expr_member))
        (sysml_decl
          (result_expr_member))
        (malformed)
        (target_transition)
        (constraint_usage
          (result_expr_member))))))
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_close_curly
semantic.duplicate_name 'normal'
semantic.unresolved_name 'TimeInstantValue'
semantic.unresolved_name 'DurationValue'
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
~~~
# FORMAT
~~~sysml
package 'Time Constraints' {
	private import ISQ::TemperatureValue;
	private import ISQ::DurationValue;
	private import Time::TimeInstantValue;
	private import Time::TimeOf;
	private import Time::DurationOf;
	private import SI::h;
	private import SI::s;

	attribute def MaintenanceDone;
	
	part def Vehicle {
		attribute maintenanceTime : TimeInstantValue;
		attribute maintenanceInterval : DurationValue;
		attribute maxTemperature : TemperatureValue;
	}
	
	state healthStates {
		in vehicle : Vehicle;
		
		entry; then normal;
		
		state normal;
		accept at vehicle.maintenanceTime
			then maintenance;
		
		state maintenance {
			assert constraint { TimeOf(maintenance) > vehicle.maintenanceTime }
			assert constraint { TimeOf(maintenance) - TimeOf(normal.done) < 2 [s] }
			entry assign vehicle.maintenanceTime := vehicle.maintenanceTime + vehicle.maintenanceInterval;
		}
		accept MaintenanceDone
			then normal;
		
		constraint { DurationOf(maintenance) <= 48 [h] }
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "0515f3aadeda18c38263cfbe353e55fb0e1b70e9980cbd8db0115d547967bc5d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Time Constraints"))) (kind "package") (name "Time Constraints") (declared-name "Time Constraints") (range (start (line 0) (character 0)) (end (line 0) (character 977))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::DurationOf"))) (kind "import") (name "DurationOf") (declared-name "DurationOf") (range (start (line 5) (character 1)) (end (line 5) (character 33))) (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::DurationOf") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::DurationValue"))) (kind "import") (name "DurationValue") (declared-name "DurationValue") (range (start (line 2) (character 1)) (end (line 2) (character 35))) (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::DurationValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::MaintenanceDone"))) (kind "attribute def") (name "MaintenanceDone") (declared-name "MaintenanceDone") (range (start (line 9) (character 1)) (end (line 9) (character 31))) (parent (node (document "d0") (qualified-name "Time Constraints"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::TemperatureValue"))) (kind "import") (name "TemperatureValue") (declared-name "TemperatureValue") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::TemperatureValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))) (kind "import") (name "TimeInstantValue") (declared-name "TimeInstantValue") (range (start (line 3) (character 1)) (end (line 3) (character 39))) (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::TimeInstantValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 38))))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::TimeOf"))) (kind "import") (name "TimeOf") (declared-name "TimeOf") (range (start (line 4) (character 1)) (end (line 4) (character 29))) (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::TimeOf") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 11) (character 1)) (end (line 11) (character 166))) (parent (node (document "d0") (qualified-name "Time Constraints"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind "attribute") (name "maintenanceInterval") (declared-name "maintenanceInterval") (range (start (line 13) (character 2)) (end (line 13) (character 48))) (parent (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DurationValue") (range none)) (typing (reference "DurationValue") (range (start (line 13) (character 34)) (end (line 13) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind "attribute") (name "maintenanceTime") (declared-name "maintenanceTime") (range (start (line 12) (character 2)) (end (line 12) (character 47))) (parent (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeInstantValue") (range none)) (typing (reference "TimeInstantValue") (range (start (line 12) (character 30)) (end (line 12) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind "attribute") (name "maxTemperature") (declared-name "maxTemperature") (range (start (line 14) (character 2)) (end (line 14) (character 46))) (parent (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureValue") (range none)) (typing (reference "TemperatureValue") (range (start (line 14) (character 29)) (end (line 14) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::h"))) (kind "import") (name "h") (declared-name "h") (range (start (line 6) (character 1)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::h") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 21))))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind "state") (name "healthStates") (declared-name "healthStates") (range (start (line 17) (character 1)) (end (line 17) (character 517))) (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Feature)) (relationships (transition (reference "Time Constraints::healthStates::maintenance") (range none)) (transition (reference "Time Constraints::healthStates::normal") (range none)) (initial-state (reference "Time Constraints::healthStates::normal") (range none)))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 20) (character 2)) (end (line 20) (character 8))) (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::maintenance"))) (kind "state") (name "maintenance") (declared-name "maintenance") (range (start (line 26) (character 2)) (end (line 26) (character 269))) (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::normal"))) (kind "state") (name "normal") (declared-name "normal") (range (start (line 22) (character 2)) (end (line 22) (character 15))) (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_maintenance"))) (kind "transition") (name "transition_healthStates_to_maintenance") (declared-name "transition_healthStates_to_maintenance") (range (start (line 23) (character 2)) (end (line 23) (character 56))) (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_maintenance::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 23) (character 2)) (end (line 23) (character 56))) (parent (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_maintenance"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_normal"))) (kind "transition") (name "transition_healthStates_to_normal") (declared-name "transition_healthStates_to_normal") (range (start (line 31) (character 2)) (end (line 31) (character 40))) (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 31) (character 2)) (end (line 31) (character 40))) (parent (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_normal"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (kind "in out parameter") (name "vehicle") (declared-name "vehicle") (range (start (line 18) (character 2)) (end (line 18) (character 23))) (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::s"))) (kind "import") (name "s") (declared-name "s") (range (start (line 7) (character 1)) (end (line 7) (character 22))) (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::s") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 21))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::DurationOf"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::DurationOf") (range (start (line 5) (character 16)) (end (line 5) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::DurationValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::DurationValue") (range (start (line 2) (character 16)) (end (line 2) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::TemperatureValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::TemperatureValue") (range (start (line 1) (character 16)) (end (line 1) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::TimeInstantValue") (range (start (line 3) (character 16)) (end (line 3) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::TimeOf"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::TimeOf") (range (start (line 4) (character 16)) (end (line 4) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 1)) (authored-target "DurationValue") (range (start (line 13) (character 34)) (end (line 13) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "TimeInstantValue") (range (start (line 12) (character 30)) (end (line 12) (character 46))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::TemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureValue") (range (start (line 14) (character 29)) (end (line 14) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::TemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::h"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::h") (range (start (line 6) (character 16)) (end (line 6) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind transitionSource) (ordinal 0)) (authored-target "Time Constraints::healthStates::maintenance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::maintenance")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind transitionSource) (ordinal 1)) (authored-target "Time Constraints::healthStates::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "Time Constraints::healthStates::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::s"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::s") (range (start (line 7) (character 16)) (end (line 7) (character 21))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (target (node (document "d0") (qualified-name "Time Constraints::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (target (node (document "d0") (qualified-name "Time Constraints::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (target (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (target (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (target (node (document "d0") (qualified-name "Time Constraints::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (target (node (document "d0") (qualified-name "Time Constraints::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::maintenance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (target (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
