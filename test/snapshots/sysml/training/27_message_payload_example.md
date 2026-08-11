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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "27_message_payload_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 18) (end 11 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 8) (end 23 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 31) (end 23 72))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "fe77d90c35754c43a7dbb1aa965db0993fcfa6b94626b02741f02e592d3e14a5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Message Payload Example"))) (kind "package") (name "Message Payload Example") (declared-name "Message Payload Example") (range (start (line 0) (character 0)) (end (line 0) (character 1141))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 46))) (parent (node (document "d0") (qualified-name "Message Payload Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Event Occurrence Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction"))) (kind "occurrence def") (name "CruiseControlInteraction") (declared-name "CruiseControlInteraction") (range (start (line 18) (character 1)) (end (line 18) (character 739))) (parent (node (document "d0") (qualified-name "Message Payload Example"))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (kind "flow") (name "setSpeedMessage") (declared-name "setSpeedMessage") (range (start (line 22) (character 2)) (end (line 22) (character 112))) (parent (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction"))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 22) (character 29)) (end (line 22) (character 37))) (parent (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (authored (relationships (typing (reference "SetSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::EngineController"))) (kind "part def") (name "EngineController") (declared-name "EngineController") (range (start (line 9) (character 1)) (end (line 9) (character 27))) (parent (node (document "d0") (qualified-name "Message Payload Example"))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::FuelCommand"))) (kind "item def") (name "FuelCommand") (declared-name "FuelCommand") (range (start (line 5) (character 1)) (end (line 5) (character 69))) (parent (node (document "d0") (qualified-name "Message Payload Example"))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::FuelCommand::fuelFlow"))) (kind "attribute") (name "fuelFlow") (declared-name "fuelFlow") (range (start (line 6) (character 2)) (end (line 6) (character 42))) (parent (node (document "d0") (qualified-name "Message Payload Example::FuelCommand"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::SensedSpeed"))) (kind "item def") (name "SensedSpeed") (declared-name "SensedSpeed") (range (start (line 4) (character 1)) (end (line 4) (character 22))) (parent (node (document "d0") (qualified-name "Message Payload Example"))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::SetSpeed"))) (kind "item def") (name "SetSpeed") (declared-name "SetSpeed") (range (start (line 3) (character 1)) (end (line 3) (character 19))) (parent (node (document "d0") (qualified-name "Message Payload Example"))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::vehicle1"))) (kind "part") (name "vehicle1") (declared-name "vehicle1") (range (start (line 11) (character 1)) (end (line 11) (character 167))) (parent (node (document "d0") (qualified-name "Message Payload Example"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 11) (character 18)) (end (line 11) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind "part") (name "engineController") (declared-name "engineController") (range (start (line 12) (character 2)) (end (line 12) (character 136))) (parent (node (document "d0") (qualified-name "Message Payload Example::vehicle1"))) (authored (membership (kind Feature)) (relationships (typing (reference "EngineController") (range (start (line 12) (character 26)) (end (line 12) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandForwarded"))) (kind "occurrence") (name "fuelCommandForwarded") (declared-name "fuelCommandForwarded") (range (start (line 14) (character 25)) (end (line 14) (character 46))) (parent (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController"))))
    (element (id (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandReceived"))) (kind "occurrence") (name "fuelCommandReceived") (declared-name "fuelCommandReceived") (range (start (line 13) (character 20)) (end (line 13) (character 40))) (parent (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Message Payload Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Event Occurrence Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction"))) (kind flowSource) (ordinal 0)) (authored-target "driver::setSpeedSent") (range (start (line 23) (character 8)) (end (line 23) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction"))) (kind flowTarget) (ordinal 0)) (authored-target "vehicle::cruiseController::setSpeedReceived") (range (start (line 23) (character 31)) (end (line 23) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Message Payload Example::SetSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Message Payload Example::FuelCommand::fuelFlow"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Message Payload Example::vehicle1"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 11) (character 18)) (end (line 11) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind featureTyping) (ordinal 0)) (authored-target "EngineController") (range (start (line 12) (character 26)) (end (line 12) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Message Payload Example::EngineController")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage::_payload"))) (target (node (document "d0") (qualified-name "Message Payload Example::SetSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController"))) (target (node (document "d0") (qualified-name "Message Payload Example::EngineController"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
