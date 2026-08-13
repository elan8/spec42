# META
~~~ini
description=SysML Training 27 (Occurrences): Interaction Realization-1
type=file
~~~
# SOURCE
~~~sysml
package 'Interaction Realization-1' {
	private import 'Interaction Example-1'::*;
	
	part driver_a : Driver {
		action driverBehavior {
			action sendSetSpeed send new SetSpeed() to vehicle_a;
		}
	}
	
	part vehicle_a : Vehicle {
		part cruiseController_a : CruiseController {
			action controllerBehavior {
				action receiveSetSpeed accept SetSpeed via vehicle_a;
				then action receiveSensedSpeed accept SensedSpeed via cruiseController_a;
				then action sendFuelCommand send new FuelCommand() to engine_a;
			}
		}
		
		part speedometer_a : Speedometer {
			action speedometerBehavior {
				action sendSensedSpeed send new SensedSpeed() to cruiseController_a;
			}
		}
		
		part engine_a : Engine {
			action engineBehavior {
				action receiveFuelCommand accept FuelCommand via engine_a;
			}
		}
	}
	
	occurrence cruiseControlInteraction_a : CruiseControlInteraction {
		part :>> driver :>> driver_a {
			event driverBehavior.sendSetSpeed[1] :>> setSpeedSent;
		}
		
		part :>> vehicle :>> vehicle_a {
			part :>> cruiseController :>> cruiseController_a {
				event controllerBehavior.receiveSetSpeed[1] :>> setSpeedReceived;
				event controllerBehavior.receiveSensedSpeed[1] :>> sensedSpeedReceived;
				event controllerBehavior.sendFuelCommand[1] :>> fuelCommandSent;
			}
			part :>> speedometer :>> speedometer_a {
				event speedometerBehavior.sendSensedSpeed[1] :>> sensedSpeedSent;
			}
			part :>> engine :>> engine_a {
				event engineBehavior.receiveFuelCommand[1] :>> fuelCommandReceived;
			}
		}
		
		message :>> setSpeedMessage = driver_a.driverBehavior.sendSetSpeed.sentMessage;
		message :>> sensedSpeedMessage = vehicle_a.speedometer_a.speedometerBehavior.sendSensedSpeed.sentMessage;
		message :>> fuelCommandMessage = vehicle_a.cruiseController_a.controllerBehavior.sendFuelCommand.sentMessage;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/27_interaction_realization_1.md"
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
        (range (start 3 17) (end 3 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 4 2) (end 6 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 18) (end 9 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 28) (end 10 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 11 3) (end 15 4))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 12 27) (end 13 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 12 27) (end 13 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 23) (end 18 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 19 3) (end 21 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 18) (end 24 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 25 3) (end 27 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 12) (end 53 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:44f0afb918cd2e030af601da4a84161de808c2966bb6c40aa4863e9d1789a493") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Interaction Example-1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driver"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseController"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Speedometer"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Interaction Example-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseController")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Speedometer")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 1 16) (end 1 42)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Interaction Example-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 3 17) (end 3 23)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a"))) (kind featureTyping) (ordinal 0) (authored-target "Driver")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 9 18) (end 9 25)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 10 28) (end 10 44)) (probe (position 10 28))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 24 18) (end 24 24)) (probe (position 24 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 18 23) (end 18 34)) (probe (position 18 23))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind featureTyping) (ordinal 0) (authored-target "Speedometer")
      (outcome (status unresolved)))
  )
)
~~~
