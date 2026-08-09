# META
~~~ini
description=SysML Training 27 (Occurrences): Message Payload Example
type=file
~~~
# SOURCE
~~~sysml
package 'Message Payload Example' {
	private import 'Event Occurrence Example'::*;
	
	item def SetSpeed;
	item def SensedSpeed;
	item def FuelCommand {
		attribute fuelFlow : ScalarValues::Real;
	}
	
	part def EngineController;
	
	part vehicle1 :> vehicle {
		part engineController : EngineController {
			event occurrence fuelCommandReceived;
			then event occurrence fuelCommandForwarded;
		}
	}
	
	occurrence def CruiseControlInteraction {		
		ref part :>> driver;		
		ref part vehicle :>> vehicle1;
		
		message setSpeedMessage of SetSpeed 
			from driver.setSpeedSent to vehicle.cruiseController.setSpeedReceived;
			
		then message sensedSpeedMessage of SensedSpeed 
			from vehicle.speedometer.sensedSpeedSent to vehicle.cruiseController.sensedSpeedReceived;
			
		then message fuelCommandMessage of fuelCommand : FuelCommand 
			from vehicle.cruiseController.fuelCommandSent to vehicle.engineController.fuelCommandReceived;
		
		then message fuelCommandForwardingMessage of fuelCommand : FuelCommand = fuelCommandMessage.fuelCommand
			from vehicle.engineController.fuelCommandForwarded to vehicle.engine.fuelCommandReceived;
		
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwOccurrence,KwDef,Ident,OpenCurly,
KwRef,KwPart,ColonGtGt,Ident,Semicolon,
KwRef,KwPart,Ident,ColonGtGt,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwThen,KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwThen,KwMessage,Ident,KwOf,Ident,Colon,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwThen,KwMessage,Ident,KwOf,Ident,Colon,Ident,Eq,Ident,Dot,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Message Payload Example''
    (import_decl private ''Event Occurrence Example'::*')
    (item_def 'SetSpeed')
    (item_def 'SensedSpeed')
    (item_def 'FuelCommand'
      (attribute_usage 'fuelFlow' : 'ScalarValues::Real'))
    (part_def 'EngineController')
    (part_usage 'vehicle1' :> 'vehicle'
      (part_usage 'engineController' : 'EngineController'
        (event_occurrence 'fuelCommandReceived')
        (source_succession
          (event_occurrence 'fuelCommandForwarded'))))
    (occurrence_def 'CruiseControlInteraction'
      (part_usage ref :>> 'driver')
      (part_usage ref 'vehicle' :>> 'vehicle1')
      (message_usage 'setSpeedMessage' : 'SetSpeed'
        (connector_end)
        (connector_end))
      (source_succession
        (message_usage 'sensedSpeedMessage' : 'SensedSpeed'
          (connector_end)
          (connector_end)))
      (source_succession
        (message_usage 'fuelCommandMessage' : 'fuelCommand'))
      (source_succession
        (message_usage 'fuelCommandForwardingMessage' : 'fuelCommand')))))
~~~
# FORMAT
~~~sysml
package 'Message Payload Example' {
    private import 'Event Occurrence Example'::*;

    item def SetSpeed;
    item def SensedSpeed;
    item def FuelCommand {
        attribute fuelFlow : ScalarValues::Real;
    }

    part def EngineController;

    part vehicle1 :> vehicle {
        part engineController : EngineController {
            event occurrence fuelCommandReceived;
            then event occurrence fuelCommandForwarded;
        }
    }

    occurrence def CruiseControlInteraction {
        ref part :>> driver;
        ref part vehicle :>> vehicle1;

        message setSpeedMessage of SetSpeed
        from driver.setSpeedSent to vehicle.cruiseController.setSpeedReceived;

        then message sensedSpeedMessage of SensedSpeed
        from vehicle.speedometer.sensedSpeedSent to vehicle.cruiseController.sensedSpeedReceived;

        then message fuelCommandMessage of fuelCommand : FuelCommand
        from vehicle.cruiseController.fuelCommandSent to vehicle.engineController.fuelCommandReceived;

        then message fuelCommandForwardingMessage of fuelCommand : FuelCommand = fuelCommandMessage.fuelCommand
        from vehicle.engineController.fuelCommandForwarded to vehicle.engine.fuelCommandReceived;

    }
}

~~~
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'vehicle'
semantic.unresolved_name 'driver'
semantic.unresolved_name 'fuelCommand'
semantic.unresolved_name 'fuelCommand'
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'vehicle'
semantic.unresolved_name 'driver'
semantic.unresolved_name 'fuelCommand'
semantic.unresolved_name 'fuelCommand'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Message Payload Example"))) (name "Message Payload Example") (declared-name "Message Payload Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Message Payload Example::*"))) (name "*") (declared-name "*"))
        (element (kind "occurrence def") (id (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction"))) (name "CruiseControlInteraction") (declared-name "CruiseControlInteraction") (declared)
          (contains
            (element (kind "flow") (id (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (name "setSpeedMessage") (declared-name "setSpeedMessage") (effective (featuring-type (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage::_payload"))) (name "_payload") (declared-name "_payload") (effective (featuring-type (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction")))))
              )
            )
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Message Payload Example::EngineController"))) (name "EngineController") (declared-name "EngineController") (declared))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Message Payload Example::FuelCommand"))) (name "FuelCommand") (declared-name "FuelCommand")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Message Payload Example::FuelCommand::fuelFlow"))) (name "fuelFlow") (declared-name "fuelFlow") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Message Payload Example::FuelCommand")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "Message Payload Example::SensedSpeed"))) (name "SensedSpeed") (declared-name "SensedSpeed"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Message Payload Example::SetSpeed"))) (name "SetSpeed") (declared-name "SetSpeed"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Message Payload Example::vehicle1"))) (name "vehicle1") (declared-name "vehicle1") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController"))) (name "engineController") (declared-name "engineController") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandForwarded"))) (name "fuelCommandForwarded") (declared-name "fuelCommandForwarded") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Message Payload Example::EngineController")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandReceived"))) (name "fuelCommandReceived") (declared-name "fuelCommandReceived") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Message Payload Example::EngineController")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage::_payload"))) (to (node (document "d0") (qualified-name "Message Payload Example::SetSpeed"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController"))) (to (node (document "d0") (qualified-name "Message Payload Example::EngineController"))))
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
  (document "sysml/training/27_message_payload_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 42))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 25 2) (end 25 149))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 28 2) (end 28 167))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 31 2) (end 31 203))
      )
    )
  )
)
~~~
