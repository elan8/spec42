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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c1c543b14fdc599849b1c61a682f454718b30f1f094c00e0cc39bfbf42240c57") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interaction Example-1"))) (kind "package") (name "Interaction Example-1") (declared-name "Interaction Example-1"))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Interaction Example-1"))) (authored (membership (kind Import) (visibility "public") (import (reference "Event Occurrence Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind "occurrence def") (name "CruiseControlInteraction") (declared-name "CruiseControlInteraction") (parent (node (document "d0") (qualified-name "Interaction Example-1"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind "flow") (name "fuelCommandMessage") (declared-name "fuelCommandMessage") (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (authored (relationships (typing (reference "FuelCommand")))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind "flow") (name "sensedSpeedMessage") (declared-name "sensedSpeedMessage") (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (authored (relationships (typing (reference "SensedSpeed")))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind "flow") (name "setSpeedMessage") (declared-name "setSpeedMessage") (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (parent (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (authored (relationships (typing (reference "SetSpeed")))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::FuelCommand"))) (kind "item def") (name "FuelCommand") (declared-name "FuelCommand") (parent (node (document "d0") (qualified-name "Interaction Example-1"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::SensedSpeed"))) (kind "item def") (name "SensedSpeed") (declared-name "SensedSpeed") (parent (node (document "d0") (qualified-name "Interaction Example-1"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-1::SetSpeed"))) (kind "item def") (name "SetSpeed") (declared-name "SetSpeed") (parent (node (document "d0") (qualified-name "Interaction Example-1"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Event Occurrence Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowSource) (ordinal 0)) (authored-target "driver::setSpeedSent") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowSource) (ordinal 1)) (authored-target "vehicle::speedometer::sensedSpeedSent") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowSource) (ordinal 2)) (authored-target "vehicle::cruiseController::fuelCommandSent") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowTarget) (ordinal 0)) (authored-target "vehicle::cruiseController::setSpeedReceived") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowTarget) (ordinal 1)) (authored-target "vehicle::cruiseController::sensedSpeedReceived") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind flowTarget) (ordinal 2)) (authored-target "vehicle::engine::fuelCommandReceived") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCommand") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Example-1::FuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "SensedSpeed") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Example-1::SensedSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Example-1::SetSpeed")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 12 8) (end 12 27)) (probe (position 12 8))
      (reference
        (source (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))
        (kind flowSource) (ordinal 0) (authored-target "driver::setSpeedSent")
        (range (start 12 8) (end 12 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 15) (end 1 41)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "Interaction Example-1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Event Occurrence Example::*")
        (range (start 1 15) (end 1 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 52) (end 18 86)) (probe (position 18 52))
      (reference
        (source (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))
        (kind flowTarget) (ordinal 2) (authored-target "vehicle::engine::fuelCommandReceived")
        (range (start 18 52) (end 18 86))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 8) (end 15 43)) (probe (position 15 8))
      (reference
        (source (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))
        (kind flowSource) (ordinal 1) (authored-target "vehicle::speedometer::sensedSpeedSent")
        (range (start 15 8) (end 15 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 8) (end 18 48)) (probe (position 18 8))
      (reference
        (source (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))
        (kind flowSource) (ordinal 2) (authored-target "vehicle::cruiseController::fuelCommandSent")
        (range (start 18 8) (end 18 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 31) (end 12 72)) (probe (position 12 31))
      (reference
        (source (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))
        (kind flowTarget) (ordinal 0) (authored-target "vehicle::cruiseController::setSpeedReceived")
        (range (start 12 31) (end 12 72))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 47) (end 15 91)) (probe (position 15 47))
      (reference
        (source (document "d0") (qualified-name "Interaction Example-1::CruiseControlInteraction"))
        (kind flowTarget) (ordinal 1) (authored-target "vehicle::cruiseController::sensedSpeedReceived")
        (range (start 15 47) (end 15 91))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
