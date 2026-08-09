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

        message setSpeedMessage of SetSpeed from driver.setSpeedSent to vehicle.cruiseController.setSpeedReceived;

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
(model
  (namespace
    (package 'Message Payload Example'
      (namespace_import private -> 'Event Occurrence Example'[unresolved])
      (item_def 'SetSpeed')
      (item_def 'SensedSpeed')
      (item_def 'FuelCommand'
        (attribute_usage composite 'fuelFlow' : 'ScalarValues::Real'[unresolved]))
      (part_def 'EngineController')
      (part_usage 'vehicle1' :> 'vehicle'[unresolved]
        (part_usage composite 'engineController' : 'Message Payload Example::EngineController'[part_def]
          (event_occurrence_usage 'fuelCommandReceived')
          (source_succession
            (event_occurrence_usage 'fuelCommandForwarded'))))
      (occurrence_def 'CruiseControlInteraction'
        (part_usage reference :>> 'driver'[unresolved])
        (part_usage reference 'vehicle' :>> 'Message Payload Example::vehicle1'[part_usage])
        (flow_usage composite 'setSpeedMessage' : 'Message Payload Example::SetSpeed'[item_def]
          (connector_end 'driver.setSpeedSent')
          (connector_end 'vehicle.cruiseController.setSpeedReceived'))
        (source_succession
          (flow_usage 'sensedSpeedMessage' : 'Message Payload Example::SensedSpeed'[item_def]
            (connector_end 'vehicle.speedometer.sensedSpeedSent')
            (connector_end 'vehicle.cruiseController.sensedSpeedReceived')))
        (source_succession
          (flow_usage 'fuelCommandMessage' : 'fuelCommand'[unresolved]))
        (source_succession
          (flow_usage 'fuelCommandForwardingMessage' : 'fuelCommand'[unresolved]))))))
~~~
