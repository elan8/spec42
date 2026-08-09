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

        entry;
        then normal;

        state normal;
        accept at vehicle . maintenanceTime then maintenance;

        state maintenance {
            assert constraint {
                = TimeOf(maintenance) > vehicle.maintenanceTime;
            }
            assert constraint {
                = TimeOf(maintenance) - TimeOf(normal.done) < 2[s];
            }
            }
            accept MaintenanceDone then normal;

            constraint {
                = DurationOf(maintenance) <= 48[h];
            }
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
(model
  (namespace
    (package 'Time Constraints'
      (membership_import private -> 'ISQ::TemperatureValue'[unresolved])
      (membership_import private -> 'ISQ::DurationValue'[unresolved])
      (membership_import private -> 'Time::TimeInstantValue'[unresolved])
      (membership_import private -> 'Time::TimeOf'[unresolved])
      (membership_import private -> 'Time::DurationOf'[unresolved])
      (membership_import private -> 'SI::h'[unresolved])
      (membership_import private -> 'SI::s'[unresolved])
      (attribute_def 'MaintenanceDone')
      (part_def 'Vehicle'
        (attribute_usage composite 'maintenanceTime' : 'TimeInstantValue'[unresolved])
        (attribute_usage composite 'maintenanceInterval' : 'DurationValue'[unresolved])
        (attribute_usage composite 'maxTemperature' : 'TemperatureValue'[unresolved]))
      (state_usage 'healthStates'
        (reference_usage in reference 'vehicle' : 'Time Constraints::Vehicle'[part_def])
        (state_subaction_membership 'entry'
          (action_usage))
        (source_succession
          (reference_usage reference 'normal'))
        (state_usage composite 'normal')
        (transition_usage)
        (state_usage composite 'maintenance'
          (assert_constraint_usage
            (result_expr_membership))
          (assert_constraint_usage
            (result_expr_membership))
          (not_implemented 'malformed')
          (transition_usage)
          (constraint_usage composite
            (result_expr_membership)))))))
~~~
