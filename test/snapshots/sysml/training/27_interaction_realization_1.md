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
  (document "27_interaction_realization_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 17) (end 3 23))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 4) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 4) (end 14 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 23) (end 18 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 18) (end 24 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 12) (end 31 1016))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 2) (end 32 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 44) (end 33 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 542))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 33) (end 37 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 38 52) (end 38 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 55) (end 39 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 40 52) (end 40 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 28) (end 42 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 53) (end 43 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 23) (end 45 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 51) (end 46 70))
      )
      (diagnostic
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "sysml")
        (range (start 50 2) (end 50 84))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 50 2) (end 50 84))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "3a6912c25428709d350dddc883c7c385054527f558e34651af1047bb76c1e76a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1"))) (kind "package") (name "Interaction Realization-1") (declared-name "Interaction Realization-1"))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Interaction Realization-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Interaction Example-1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (kind "occurrence") (name "cruiseControlInteraction_a") (declared-name "cruiseControlInteraction_a") (parent (node (document "d0") (qualified-name "Interaction Realization-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseControlInteraction")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (kind "part") (name "driver") (declared-name "driver") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver::driverBehavior.sendSetSpeed"))) (kind "occurrence") (name "driverBehavior.sendSetSpeed") (declared-name "driverBehavior.sendSetSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "setSpeedSent")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (kind "part") (name "cruiseController") (declared-name "cruiseController") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cruiseController_a")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSensedSpeed"))) (kind "occurrence") (name "controllerBehavior.receiveSensedSpeed") (declared-name "controllerBehavior.receiveSensedSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sensedSpeedReceived")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSetSpeed"))) (kind "occurrence") (name "controllerBehavior.receiveSetSpeed") (declared-name "controllerBehavior.receiveSetSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "setSpeedReceived")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.sendFuelCommand"))) (kind "occurrence") (name "controllerBehavior.sendFuelCommand") (declared-name "controllerBehavior.sendFuelCommand") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCommandSent")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine_a")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine::engineBehavior.receiveFuelCommand"))) (kind "occurrence") (name "engineBehavior.receiveFuelCommand") (declared-name "engineBehavior.receiveFuelCommand") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCommandReceived")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (kind "part") (name "speedometer") (declared-name "speedometer") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "speedometer_a")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer::speedometerBehavior.sendSensedSpeed"))) (kind "occurrence") (name "speedometerBehavior.sendSensedSpeed") (declared-name "speedometerBehavior.sendSensedSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sensedSpeedSent")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::driver_a"))) (kind "part") (name "driver_a") (declared-name "driver_a") (parent (node (document "d0") (qualified-name "Interaction Realization-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driver")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))) (kind "action") (name "driverBehavior") (declared-name "driverBehavior") (parent (node (document "d0") (qualified-name "Interaction Realization-1::driver_a"))) (authored (membership (kind Feature)) (relationships (perform (reference "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (kind "action") (name "sendSetSpeed") (declared-name "sendSetSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind "part") (name "vehicle_a") (declared-name "vehicle_a") (parent (node (document "d0") (qualified-name "Interaction Realization-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind "part") (name "cruiseController_a") (declared-name "cruiseController_a") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseController")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind "action") (name "controllerBehavior") (declared-name "controllerBehavior") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (authored (membership (kind Feature)) (relationships (perform (reference "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed")) (perform (reference "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed")) (perform (reference "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (kind "action") (name "receiveSensedSpeed") (declared-name "receiveSensedSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (authored (relationships (typing (reference "")) (flow (reference "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed"))) (kind "action") (name "receiveSetSpeed") (declared-name "receiveSetSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind "action") (name "sendFuelCommand") (declared-name "sendFuelCommand") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind "part") (name "engine_a") (declared-name "engine_a") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))) (kind "action") (name "engineBehavior") (declared-name "engineBehavior") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (authored (membership (kind Feature)) (relationships (perform (reference "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand"))) (kind "action") (name "receiveFuelCommand") (declared-name "receiveFuelCommand") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind "part") (name "speedometer_a") (declared-name "speedometer_a") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "Speedometer")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))) (kind "action") (name "speedometerBehavior") (declared-name "speedometerBehavior") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (authored (membership (kind Feature)) (relationships (perform (reference "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (kind "action") (name "sendSensedSpeed") (declared-name "sendSensedSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Interaction Example-1::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseControlInteraction") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver::driverBehavior.sendSetSpeed"))) (kind redefinition) (ordinal 0)) (authored-target "setSpeedSent") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (kind redefinition) (ordinal 0)) (authored-target "cruiseController_a") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSensedSpeed"))) (kind redefinition) (ordinal 0)) (authored-target "sensedSpeedReceived") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSetSpeed"))) (kind redefinition) (ordinal 0)) (authored-target "setSpeedReceived") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.sendFuelCommand"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCommandSent") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine_a") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine::engineBehavior.receiveFuelCommand"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCommandReceived") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (kind redefinition) (ordinal 0)) (authored-target "speedometer_a") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer::speedometerBehavior.sendSensedSpeed"))) (kind redefinition) (ordinal 0)) (authored-target "sensedSpeedSent") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::driver_a"))) (kind featureTyping) (ordinal 0)) (authored-target "Driver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))) (kind performSource) (ordinal 0)) (authored-target "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseController") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind performSource) (ordinal 0)) (authored-target "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind performSource) (ordinal 1)) (authored-target "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind performSource) (ordinal 2)) (authored-target "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (kind flowSource) (ordinal 0)) (authored-target "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))) (kind performSource) (ordinal 0)) (authored-target "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind featureTyping) (ordinal 0)) (authored-target "Speedometer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))) (kind performSource) (ordinal 0)) (authored-target "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed")))))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))) (target (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind performSource) (ordinal 2)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (kind flowSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))) (kind performSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 17) (end 3 23)) (probe (position 3 17))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::driver_a"))
        (kind featureTyping) (ordinal 0) (authored-target "Driver")
        (range (start 3 17) (end 3 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 18) (end 24 24)) (probe (position 24 18))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 24 18) (end 24 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 18) (end 9 25)) (probe (position 9 18))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 9 18) (end 9 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 23) (end 45 31)) (probe (position 45 23))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))
        (kind redefinition) (ordinal 0) (authored-target "engine_a")
        (range (start 45 23) (end 45 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 23) (end 18 34)) (probe (position 18 23))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))
        (kind featureTyping) (ordinal 0) (authored-target "Speedometer")
        (range (start 18 23) (end 18 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 44) (end 33 56)) (probe (position 33 44))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver::driverBehavior.sendSetSpeed"))
        (kind redefinition) (ordinal 0) (authored-target "setSpeedSent")
        (range (start 33 44) (end 33 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 42 28) (end 42 41)) (probe (position 42 28))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))
        (kind redefinition) (ordinal 0) (authored-target "speedometer_a")
        (range (start 42 28) (end 42 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 52) (end 40 67)) (probe (position 40 52))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.sendFuelCommand"))
        (kind redefinition) (ordinal 0) (authored-target "fuelCommandSent")
        (range (start 40 52) (end 40 67))
        (outcome (status unresolved))
      )
    )
    (query (range (start 43 53) (end 43 68)) (probe (position 43 53))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer::speedometerBehavior.sendSensedSpeed"))
        (kind redefinition) (ordinal 0) (authored-target "sensedSpeedSent")
        (range (start 43 53) (end 43 68))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 28) (end 10 44)) (probe (position 10 28))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))
        (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
        (range (start 10 28) (end 10 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 38 52) (end 38 68)) (probe (position 38 52))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSetSpeed"))
        (kind redefinition) (ordinal 0) (authored-target "setSpeedReceived")
        (range (start 38 52) (end 38 68))
        (outcome (status unresolved))
      )
    )
    (query (range (start 37 33) (end 37 51)) (probe (position 37 33))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))
        (kind redefinition) (ordinal 0) (authored-target "cruiseController_a")
        (range (start 37 33) (end 37 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 39 55) (end 39 74)) (probe (position 39 55))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSensedSpeed"))
        (kind redefinition) (ordinal 0) (authored-target "sensedSpeedReceived")
        (range (start 39 55) (end 39 74))
        (outcome (status unresolved))
      )
    )
    (query (range (start 46 51) (end 46 70)) (probe (position 46 51))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine::engineBehavior.receiveFuelCommand"))
        (kind redefinition) (ordinal 0) (authored-target "fuelCommandReceived")
        (range (start 46 51) (end 46 70))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 39)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Interaction Example-1::*")
        (range (start 1 16) (end 1 39))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
