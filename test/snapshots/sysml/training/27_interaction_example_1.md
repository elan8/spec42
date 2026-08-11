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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "27_interaction_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 8) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 31) (end 12 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 8) (end 15 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 47) (end 15 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 8) (end 18 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 52) (end 18 86))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a2527dcb86ba89c14ee31973ae6b45d0db1dd01b4025f21e39f6c075557d9794") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interaction Example-1"))) (kind "package") (name "Interaction Example-1") (declared-name "Interaction Example-1") (range (start (line 0) (character 0)) (end (line 0) (character 695))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 45))) (parent (node (document "d0") (qualified-name "Interaction Example-1"))) (authored (membership (kind Import) (visibility "public") (import (reference "Event Occurrence Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind "occurrence def") (name "CruiseControlInteraction") (declared-name "CruiseControlInteraction") (range (start (line 7) (character 1)) (end (line 7) (character 543))) (parent (node (document "d0") (qualified-name "Interaction Example-1"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind "flow") (name "fuelCommandMessage") (declared-name "fuelCommandMessage") (range (start (line 17) (character 2)) (end (line 17) (character 132))) (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 17) (character 32)) (end (line 17) (character 43))) (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (authored (relationships (typing (reference "FuelCommand") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind "flow") (name "sensedSpeedMessage") (declared-name "sensedSpeedMessage") (range (start (line 14) (character 2)) (end (line 14) (character 137))) (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 14) (character 32)) (end (line 14) (character 43))) (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (authored (relationships (typing (reference "SensedSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind "flow") (name "setSpeedMessage") (declared-name "setSpeedMessage") (range (start (line 11) (character 2)) (end (line 11) (character 112))) (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 11) (character 29)) (end (line 11) (character 37))) (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (authored (relationships (typing (reference "SetSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::FuelCommand"))) (kind "item def") (name "FuelCommand") (declared-name "FuelCommand") (range (start (line 5) (character 1)) (end (line 5) (character 22))) (parent (node (document "d0") (qualified-name "Interaction Example-1"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::SensedSpeed"))) (kind "item def") (name "SensedSpeed") (declared-name "SensedSpeed") (range (start (line 4) (character 1)) (end (line 4) (character 22))) (parent (node (document "d0") (qualified-name "Interaction Example-1"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::SetSpeed"))) (kind "item def") (name "SetSpeed") (declared-name "SetSpeed") (range (start (line 3) (character 1)) (end (line 3) (character 19))) (parent (node (document "d0") (qualified-name "Interaction Example-1"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Event Occurrence Example::*") (range (start (line 1) (character 15)) (end (line 1) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowSource) (ordinal 0)) (authored-target "driver::setSpeedSent") (range (start (line 12) (character 8)) (end (line 12) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowSource) (ordinal 1)) (authored-target "vehicle::speedometer::sensedSpeedSent") (range (start (line 15) (character 8)) (end (line 15) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowSource) (ordinal 2)) (authored-target "vehicle::cruiseController::fuelCommandSent") (range (start (line 18) (character 8)) (end (line 18) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowTarget) (ordinal 0)) (authored-target "vehicle::cruiseController::setSpeedReceived") (range (start (line 12) (character 31)) (end (line 12) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowTarget) (ordinal 1)) (authored-target "vehicle::cruiseController::sensedSpeedReceived") (range (start (line 15) (character 47)) (end (line 15) (character 91))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowTarget) (ordinal 2)) (authored-target "vehicle::engine::fuelCommandReceived") (range (start (line 18) (character 52)) (end (line 18) (character 86))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCommand") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Example-1::FuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "SensedSpeed") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Example-1::SensedSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Example-1::SetSpeed")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage::_payload"))) (target (node (document "d0") (qualified-name "Interaction Example-1::FuelCommand"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage::_payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage::_payload"))) (target (node (document "d0") (qualified-name "Interaction Example-1::SensedSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage::_payload"))) (target (node (document "d0") (qualified-name "Interaction Example-1::SetSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
