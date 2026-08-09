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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Time Constraints"))) (name "Time Constraints") (declared-name "Time Constraints")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Time Constraints::DurationOf"))) (name "DurationOf") (declared-name "DurationOf"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time Constraints::DurationValue"))) (name "DurationValue") (declared-name "DurationValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time Constraints::MaintenanceDone"))) (name "MaintenanceDone") (declared-name "MaintenanceDone") (declared (properties (ordered false) (unique true))))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time Constraints::TemperatureValue"))) (name "TemperatureValue") (declared-name "TemperatureValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))) (name "TimeInstantValue") (declared-name "TimeInstantValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Time Constraints::TimeOf"))) (name "TimeOf") (declared-name "TimeOf"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (name "maintenanceInterval") (declared-name "maintenanceInterval") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Time Constraints::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (name "maintenanceTime") (declared-name "maintenanceTime") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Time Constraints::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (name "maxTemperature") (declared-name "maxTemperature") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Time Constraints::Vehicle")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Time Constraints::h"))) (name "h") (declared-name "h"))
        (element (kind "state") (id (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (name "healthStates") (declared-name "healthStates") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Time Constraints::healthStates::_entry"))) (name "entry") (declared-name "entry"))
            (element (kind "state") (id (node (document "d0") (qualified-name "Time Constraints::healthStates::maintenance"))) (name "maintenance") (declared-name "maintenance"))
            (element (kind "state") (id (node (document "d0") (qualified-name "Time Constraints::healthStates::normal"))) (name "normal") (declared-name "normal"))
            (element (kind "transition") (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_maintenance"))) (name "transition_healthStates_to_maintenance") (declared-name "transition_healthStates_to_maintenance")
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_maintenance::trigger"))) (name "trigger") (declared-name "trigger"))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_normal"))) (name "transition_healthStates_to_normal") (declared-name "transition_healthStates_to_normal")
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_normal::trigger"))) (name "trigger") (declared-name "trigger"))
              )
            )
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Time Constraints::s"))) (name "s") (declared-name "s"))
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (to (node (document "d0") (qualified-name "Time Constraints::healthStates::normal"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (to (node (document "d0") (qualified-name "Time Constraints::healthStates::maintenance"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (to (node (document "d0") (qualified-name "Time Constraints::healthStates::normal"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (to (node (document "d0") (qualified-name "Time Constraints::Vehicle"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/31_time_constraints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 1) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 1) (end 4 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 1) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 1) (end 6 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 2) (end 13 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 46))
      )
    )
  )
)
~~~
