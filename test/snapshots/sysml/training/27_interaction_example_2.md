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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "27_interaction_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 42))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c06e73a7887c1631d274c208e54cb6662dd079021f340d5ff2917efe0576297b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interaction Example-2"))) (kind "package") (name "Interaction Example-2") (declared-name "Interaction Example-2") (range (start (line 0) (character 0)) (end (line 0) (character 800))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 46))) (parent (node (document "d0") (qualified-name "Interaction Example-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Event Occurrence Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction"))) (kind "occurrence def") (name "CruiseControlInteraction") (declared-name "CruiseControlInteraction") (range (start (line 7) (character 1)) (end (line 7) (character 647))) (parent (node (document "d0") (qualified-name "Interaction Example-2"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage"))) (kind "flow") (name "fuelCommandMessage") (declared-name "fuelCommandMessage") (range (start (line 31) (character 2)) (end (line 31) (character 44))) (parent (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 31) (character 32)) (end (line 31) (character 43))) (parent (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage"))) (authored (relationships (typing (reference "FuelCommand") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage"))) (kind "flow") (name "setSpeedMessage") (declared-name "setSpeedMessage") (range (start (line 29) (character 2)) (end (line 29) (character 38))) (parent (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 29) (character 29)) (end (line 29) (character 37))) (parent (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage"))) (authored (relationships (typing (reference "SetSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-2::FuelCommand"))) (kind "item def") (name "FuelCommand") (declared-name "FuelCommand") (range (start (line 5) (character 1)) (end (line 5) (character 22))) (parent (node (document "d0") (qualified-name "Interaction Example-2"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-2::SensedSpeed"))) (kind "item def") (name "SensedSpeed") (declared-name "SensedSpeed") (range (start (line 4) (character 1)) (end (line 4) (character 22))) (parent (node (document "d0") (qualified-name "Interaction Example-2"))))
    (element (id (node (document "d0") (qualified-name "Interaction Example-2::SetSpeed"))) (kind "item def") (name "SetSpeed") (declared-name "SetSpeed") (range (start (line 3) (character 1)) (end (line 3) (character 19))) (parent (node (document "d0") (qualified-name "Interaction Example-2"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Event Occurrence Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCommand") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Example-2::FuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Example-2::SetSpeed")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage::_payload"))) (target (node (document "d0") (qualified-name "Interaction Example-2::FuelCommand"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage::_payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage::_payload"))) (target (node (document "d0") (qualified-name "Interaction Example-2::SetSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage::_payload"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 42)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Interaction Example-2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Event Occurrence Example::*")
        (range (start 1 16) (end 1 42))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
