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
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1"))) (kind "package") (name "Interaction Realization-1") (declared-name "Interaction Realization-1") (range (start (line 0) (character 0)) (end (line 0) (character 1827))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 43))) (parent (node (document "d0") (qualified-name "Interaction Realization-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Interaction Example-1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (kind "occurrence") (name "cruiseControlInteraction_a") (declared-name "cruiseControlInteraction_a") (range (start (line 31) (character 12)) (end (line 31) (character 1016))) (parent (node (document "d0") (qualified-name "Interaction Realization-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseControlInteraction") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (kind "part") (name "driver") (declared-name "driver") (range (start (line 32) (character 2)) (end (line 32) (character 94))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver::driverBehavior.sendSetSpeed"))) (kind "occurrence") (name "driverBehavior.sendSetSpeed") (declared-name "driverBehavior.sendSetSpeed") (range (start (line 33) (character 9)) (end (line 33) (character 57))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "setSpeedSent") (range (start (line 33) (character 44)) (end (line 33) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 36) (character 2)) (end (line 36) (character 542))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (kind "part") (name "cruiseController") (declared-name "cruiseController") (range (start (line 37) (character 3)) (end (line 37) (character 273))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cruiseController_a") (range (start (line 37) (character 33)) (end (line 37) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSensedSpeed"))) (kind "occurrence") (name "controllerBehavior.receiveSensedSpeed") (declared-name "controllerBehavior.receiveSensedSpeed") (range (start (line 39) (character 10)) (end (line 39) (character 75))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sensedSpeedReceived") (range (start (line 39) (character 55)) (end (line 39) (character 74)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSetSpeed"))) (kind "occurrence") (name "controllerBehavior.receiveSetSpeed") (declared-name "controllerBehavior.receiveSetSpeed") (range (start (line 38) (character 10)) (end (line 38) (character 69))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "setSpeedReceived") (range (start (line 38) (character 52)) (end (line 38) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.sendFuelCommand"))) (kind "occurrence") (name "controllerBehavior.sendFuelCommand") (declared-name "controllerBehavior.sendFuelCommand") (range (start (line 40) (character 10)) (end (line 40) (character 68))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCommandSent") (range (start (line 40) (character 52)) (end (line 40) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 45) (character 3)) (end (line 45) (character 110))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine_a") (range (start (line 45) (character 23)) (end (line 45) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine::engineBehavior.receiveFuelCommand"))) (kind "occurrence") (name "engineBehavior.receiveFuelCommand") (declared-name "engineBehavior.receiveFuelCommand") (range (start (line 46) (character 10)) (end (line 46) (character 71))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCommandReceived") (range (start (line 46) (character 51)) (end (line 46) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (kind "part") (name "speedometer") (declared-name "speedometer") (range (start (line 42) (character 3)) (end (line 42) (character 118))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "speedometer_a") (range (start (line 42) (character 28)) (end (line 42) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer::speedometerBehavior.sendSensedSpeed"))) (kind "occurrence") (name "speedometerBehavior.sendSensedSpeed") (declared-name "speedometerBehavior.sendSensedSpeed") (range (start (line 43) (character 10)) (end (line 43) (character 69))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sensedSpeedSent") (range (start (line 43) (character 53)) (end (line 43) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::driver_a"))) (kind "part") (name "driver_a") (declared-name "driver_a") (range (start (line 3) (character 1)) (end (line 3) (character 115))) (parent (node (document "d0") (qualified-name "Interaction Realization-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driver") (range (start (line 3) (character 17)) (end (line 3) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))) (kind "action") (name "driverBehavior") (declared-name "driverBehavior") (range (start (line 4) (character 2)) (end (line 4) (character 86))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::driver_a"))) (authored (membership (kind Feature)) (relationships (perform (reference "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (kind "action") (name "sendSetSpeed") (declared-name "sendSetSpeed") (range (start (line 5) (character 3)) (end (line 5) (character 23))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind "part") (name "vehicle_a") (declared-name "vehicle_a") (range (start (line 9) (character 1)) (end (line 9) (character 604))) (parent (node (document "d0") (qualified-name "Interaction Realization-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 9) (character 18)) (end (line 9) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind "part") (name "cruiseController_a") (declared-name "cruiseController_a") (range (start (line 10) (character 2)) (end (line 10) (character 290))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseController") (range (start (line 10) (character 28)) (end (line 10) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind "action") (name "controllerBehavior") (declared-name "controllerBehavior") (range (start (line 11) (character 3)) (end (line 11) (character 239))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (authored (membership (kind Feature)) (relationships (perform (reference "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed") (range none)) (perform (reference "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed") (range none)) (perform (reference "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (kind "action") (name "receiveSensedSpeed") (declared-name "receiveSensedSpeed") (range (start (line 13) (character 4)) (end (line 13) (character 35))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (authored (relationships (typing (reference "") (range none)) (flow (reference "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed"))) (kind "action") (name "receiveSetSpeed") (declared-name "receiveSetSpeed") (range (start (line 12) (character 4)) (end (line 12) (character 27))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind "action") (name "sendFuelCommand") (declared-name "sendFuelCommand") (range (start (line 14) (character 4)) (end (line 14) (character 32))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind "part") (name "engine_a") (declared-name "engine_a") (range (start (line 24) (character 2)) (end (line 24) (character 125))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 24) (character 18)) (end (line 24) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))) (kind "action") (name "engineBehavior") (declared-name "engineBehavior") (range (start (line 25) (character 3)) (end (line 25) (character 94))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (authored (membership (kind Feature)) (relationships (perform (reference "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand"))) (kind "action") (name "receiveFuelCommand") (declared-name "receiveFuelCommand") (range (start (line 26) (character 4)) (end (line 26) (character 30))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind "part") (name "speedometer_a") (declared-name "speedometer_a") (range (start (line 18) (character 2)) (end (line 18) (character 150))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (authored (membership (kind Feature)) (relationships (typing (reference "Speedometer") (range (start (line 18) (character 23)) (end (line 18) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))) (kind "action") (name "speedometerBehavior") (declared-name "speedometerBehavior") (range (start (line 19) (character 3)) (end (line 19) (character 109))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (authored (membership (kind Feature)) (relationships (perform (reference "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (kind "action") (name "sendSensedSpeed") (declared-name "sendSensedSpeed") (range (start (line 20) (character 4)) (end (line 20) (character 27))) (parent (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Interaction Example-1::*") (range (start (line 1) (character 16)) (end (line 1) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseControlInteraction") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver::driverBehavior.sendSetSpeed"))) (kind redefinition) (ordinal 0)) (authored-target "setSpeedSent") (range (start (line 33) (character 44)) (end (line 33) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (kind redefinition) (ordinal 0)) (authored-target "cruiseController_a") (range (start (line 37) (character 33)) (end (line 37) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSensedSpeed"))) (kind redefinition) (ordinal 0)) (authored-target "sensedSpeedReceived") (range (start (line 39) (character 55)) (end (line 39) (character 74))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSetSpeed"))) (kind redefinition) (ordinal 0)) (authored-target "setSpeedReceived") (range (start (line 38) (character 52)) (end (line 38) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.sendFuelCommand"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCommandSent") (range (start (line 40) (character 52)) (end (line 40) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine_a") (range (start (line 45) (character 23)) (end (line 45) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine::engineBehavior.receiveFuelCommand"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCommandReceived") (range (start (line 46) (character 51)) (end (line 46) (character 70))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (kind redefinition) (ordinal 0)) (authored-target "speedometer_a") (range (start (line 42) (character 28)) (end (line 42) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer::speedometerBehavior.sendSensedSpeed"))) (kind redefinition) (ordinal 0)) (authored-target "sensedSpeedSent") (range (start (line 43) (character 53)) (end (line 43) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::driver_a"))) (kind featureTyping) (ordinal 0)) (authored-target "Driver") (range (start (line 3) (character 17)) (end (line 3) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))) (kind performSource) (ordinal 0)) (authored-target "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 9) (character 18)) (end (line 9) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseController") (range (start (line 10) (character 28)) (end (line 10) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind performSource) (ordinal 0)) (authored-target "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind performSource) (ordinal 1)) (authored-target "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind performSource) (ordinal 2)) (authored-target "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (kind flowSource) (ordinal 0)) (authored-target "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 24) (character 18)) (end (line 24) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))) (kind performSource) (ordinal 0)) (authored-target "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind featureTyping) (ordinal 0)) (authored-target "Speedometer") (range (start (line 18) (character 23)) (end (line 18) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))) (kind performSource) (ordinal 0)) (authored-target "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed")))))
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
