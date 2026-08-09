# META
~~~ini
description=SysML Training 27 (Occurrences): Interaction Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Interaction Example-2' {
	private import 'Event Occurrence Example'::*;
	
	item def SetSpeed;
	item def SensedSpeed;
	item def FuelCommand;
	
	occurrence def CruiseControlInteraction {
		
		ref part driver : Driver {
			event setSpeedMessage.sourceEvent;
		}
		
		ref part vehicle : Vehicle {
			part cruiseController : CruiseController {
				event setSpeedMessage.targetEvent;		
				then event sensedSpeedMessage.targetEvent;		
				then event fuelCommandMessage.sourceEvent;
			}
			
			part speedometer : Speedometer {
				event sensedSpeedMessage.sourceEvent;
			}
			
			part engine : Engine {
				event fuelCommandMessage.targetEvent;
			}
		}
		
		message setSpeedMessage of SetSpeed;	
		then message sensedSpeedMessage of SensedSpeed;
		message fuelCommandMessage of FuelCommand;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwOccurrence,KwDef,Ident,OpenCurly,
KwRef,KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRef,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwMessage,Ident,KwOf,Ident,Semicolon,
KwThen,KwMessage,Ident,KwOf,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Interaction Example-2''
    (import_decl private ''Event Occurrence Example'::*')
    (item_def 'SetSpeed')
    (item_def 'SensedSpeed')
    (item_def 'FuelCommand')
    (occurrence_def 'CruiseControlInteraction'
      (part_usage ref 'driver' : 'Driver'
        (malformed))
      (part_usage ref 'vehicle' : 'Vehicle'
        (part_usage 'cruiseController' : 'CruiseController'
          (malformed)
          (source_succession
            (malformed))
          (source_succession
            (malformed)))
        (part_usage 'speedometer' : 'Speedometer'
          (malformed))
        (part_usage 'engine' : 'Engine'
          (malformed)))
      (message_usage 'setSpeedMessage' : 'SetSpeed')
      (source_succession
        (message_usage 'sensedSpeedMessage' : 'SensedSpeed'))
      (message_usage 'fuelCommandMessage' : 'FuelCommand'))))
~~~
# FORMAT
~~~sysml
package 'Interaction Example-2' {
	private import 'Event Occurrence Example'::*;
	
	item def SetSpeed;
	item def SensedSpeed;
	item def FuelCommand;
	
	occurrence def CruiseControlInteraction {
		
		ref part driver : Driver {
			event setSpeedMessage.sourceEvent;
		}
		
		ref part vehicle : Vehicle {
			part cruiseController : CruiseController {
				event setSpeedMessage.targetEvent;		
				then event sensedSpeedMessage.targetEvent;		
				then event fuelCommandMessage.sourceEvent;
			}
			
			part speedometer : Speedometer {
				event sensedSpeedMessage.sourceEvent;
			}
			
			part engine : Engine {
				event fuelCommandMessage.targetEvent;
			}
		}
		
		message setSpeedMessage of SetSpeed;	
		then message sensedSpeedMessage of SensedSpeed;
		message fuelCommandMessage of FuelCommand;
	}
}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'Driver'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'CruiseController'
semantic.unresolved_name 'Speedometer'
semantic.unresolved_name 'Engine'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'Driver'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'CruiseController'
semantic.unresolved_name 'Speedometer'
semantic.unresolved_name 'Engine'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Interaction Example-2"))) (name "Interaction Example-2") (declared-name "Interaction Example-2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Interaction Example-2::*"))) (name "*") (declared-name "*"))
        (element (kind "occurrence def") (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction"))) (name "CruiseControlInteraction") (declared-name "CruiseControlInteraction") (declared)
          (contains
            (element (kind "flow") (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage"))) (name "fuelCommandMessage") (declared-name "fuelCommandMessage") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage::_payload"))) (name "_payload") (declared-name "_payload") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage"))) (name "setSpeedMessage") (declared-name "setSpeedMessage") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage::_payload"))) (name "_payload") (declared-name "_payload") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction")))))
              )
            )
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "Interaction Example-2::FuelCommand"))) (name "FuelCommand") (declared-name "FuelCommand"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Interaction Example-2::SensedSpeed"))) (name "SensedSpeed") (declared-name "SensedSpeed"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Interaction Example-2::SetSpeed"))) (name "SetSpeed") (declared-name "SetSpeed"))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage::_payload"))) (to (node (document "d0") (qualified-name "Interaction Example-2::FuelCommand"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage::_payload"))) (to (node (document "d0") (qualified-name "Interaction Example-2::SetSpeed"))))
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
  (document "sysml/training/27_interaction_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 46))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 30 2) (end 30 52))
      )
    )
  )
)
~~~
