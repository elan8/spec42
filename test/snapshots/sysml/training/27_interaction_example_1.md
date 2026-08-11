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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Interaction Example-1"))) (name "Interaction Example-1") (declared-name "Interaction Example-1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Interaction Example-1::*"))) (name "*") (declared-name "*"))
        (element (kind "occurrence def") (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (name "CruiseControlInteraction") (declared-name "CruiseControlInteraction") (declared)
          (contains
            (element (kind "flow") (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (name "fuelCommandMessage") (declared-name "fuelCommandMessage") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage::_payload"))) (name "_payload") (declared-name "_payload") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (name "sensedSpeedMessage") (declared-name "sensedSpeedMessage") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage::_payload"))) (name "_payload") (declared-name "_payload") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (name "setSpeedMessage") (declared-name "setSpeedMessage") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage::_payload"))) (name "_payload") (declared-name "_payload") (effective (featuring-type (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction")))))
              )
            )
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "Interaction Example-1::FuelCommand"))) (name "FuelCommand") (declared-name "FuelCommand"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Interaction Example-1::SensedSpeed"))) (name "SensedSpeed") (declared-name "SensedSpeed"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Interaction Example-1::SetSpeed"))) (name "SetSpeed") (declared-name "SetSpeed"))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage::_payload"))) (to (node (document "d0") (qualified-name "Interaction Example-1::FuelCommand"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage::_payload"))) (to (node (document "d0") (qualified-name "Interaction Example-1::SensedSpeed"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage::_payload"))) (to (node (document "d0") (qualified-name "Interaction Example-1::SetSpeed"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (status missing-prerequisite) (target "Flows::messages"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (status missing-prerequisite) (target "Flows::messages"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (status missing-prerequisite) (target "Flows::messages"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interaction Example-1::FuelCommand"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interaction Example-1::SensedSpeed"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interaction Example-1::SetSpeed"))) (status missing-prerequisite) (target "Items::Item"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/27_interaction_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 41))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 20 2) (end 20 50))
      )
    )
  )
)
~~~
