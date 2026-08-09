# META
~~~ini
description=SysML Training 27 (Occurrences): Interaction Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Interaction Example-1' {
	public import 'Event Occurrence Example'::*;
	
	item def SetSpeed;
	item def SensedSpeed;
	item def FuelCommand;
	
	occurrence def CruiseControlInteraction {		
		ref part :>> driver;		
		ref part :>> vehicle;
		
		message setSpeedMessage of SetSpeed 
			from driver.setSpeedSent to vehicle.cruiseController.setSpeedReceived;
			
		message sensedSpeedMessage of SensedSpeed 
			from vehicle.speedometer.sensedSpeedSent to vehicle.cruiseController.sensedSpeedReceived;
			
		message fuelCommandMessage of FuelCommand 
			from vehicle.cruiseController.fuelCommandSent to vehicle.engine.fuelCommandReceived;
		
		first setSpeedMessage then sensedSpeedMessage;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwOccurrence,KwDef,Ident,OpenCurly,
KwRef,KwPart,ColonGtGt,Ident,Semicolon,
KwRef,KwPart,ColonGtGt,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Interaction Example-1''
    (import_decl public ''Event Occurrence Example'::*')
    (item_def 'SetSpeed')
    (item_def 'SensedSpeed')
    (item_def 'FuelCommand')
    (occurrence_def 'CruiseControlInteraction'
      (part_usage ref :>> 'driver')
      (part_usage ref :>> 'vehicle')
      (message_usage 'setSpeedMessage' : 'SetSpeed'
        (connector_end)
        (connector_end))
      (message_usage 'sensedSpeedMessage' : 'SensedSpeed'
        (connector_end)
        (connector_end))
      (message_usage 'fuelCommandMessage' : 'FuelCommand'
        (connector_end)
        (connector_end))
      (succession_as_usage
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package 'Interaction Example-1' {
    public import 'Event Occurrence Example'::*;

    item def SetSpeed;
    item def SensedSpeed;
    item def FuelCommand;

    occurrence def CruiseControlInteraction {
        ref part :>> driver;
        ref part :>> vehicle;

        message setSpeedMessage of SetSpeed from driver.setSpeedSent to vehicle.cruiseController.setSpeedReceived;

        message sensedSpeedMessage of SensedSpeed from vehicle.speedometer.sensedSpeedSent to vehicle.cruiseController.sensedSpeedReceived;

        message fuelCommandMessage of FuelCommand from vehicle.cruiseController.fuelCommandSent to vehicle.engine.fuelCommandReceived;

        first setSpeedMessage then sensedSpeedMessage;
    }
}
~~~
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'driver'
semantic.unresolved_name 'vehicle'
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'driver'
semantic.unresolved_name 'vehicle'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Interaction Example-1'
      (namespace_import public -> 'Event Occurrence Example'[unresolved])
      (item_def 'SetSpeed')
      (item_def 'SensedSpeed')
      (item_def 'FuelCommand')
      (occurrence_def 'CruiseControlInteraction'
        (part_usage reference :>> 'driver'[unresolved])
        (part_usage reference :>> 'vehicle'[unresolved])
        (flow_usage composite 'setSpeedMessage' : 'Interaction Example-1::SetSpeed'[item_def]
          (connector_end 'driver.setSpeedSent')
          (connector_end 'vehicle.cruiseController.setSpeedReceived'))
        (flow_usage composite 'sensedSpeedMessage' : 'Interaction Example-1::SensedSpeed'[item_def]
          (connector_end 'vehicle.speedometer.sensedSpeedSent')
          (connector_end 'vehicle.cruiseController.sensedSpeedReceived'))
        (flow_usage composite 'fuelCommandMessage' : 'Interaction Example-1::FuelCommand'[item_def]
          (connector_end 'vehicle.cruiseController.fuelCommandSent')
          (connector_end 'vehicle.engine.fuelCommandReceived'))
        (succession_def
          (connector_end 'setSpeedMessage')
          (connector_end 'sensedSpeedMessage'))))))
~~~
