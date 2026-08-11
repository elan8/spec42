# META
~~~ini
description=SysML Training 27 (Occurrences): Interaction Realization-2
type=file
~~~
# SOURCE
~~~sysml
package 'Interaction Realization-2' {
	private import 'Interaction Example-1'::*;
	
	part driver_b : Driver {
		port setSpeedPort {
			out setSpeed : SetSpeed;
		}
	}
	
	interface driverToVehicleInterface connect driver_b.setSpeedPort to vehicle_b.setSpeedPort {
		flow setSpeedFlow of SetSpeed 
			from driver_b.setSpeedPort.setSpeed to vehicle_b.setSpeedPort.setSpeed;
	}
	
	part vehicle_b : Vehicle {
		port setSpeedPort {
			in setSpeed : SetSpeed;
		}
		
		bind setSpeedPort = cruiseController_b.setSpeedPort;
		
		part cruiseController_b : CruiseController {
			port setSpeedPort {
				in setSpeed : SetSpeed;
			}
			port sensedSpeedPort {
				in sensedSpeed : SensedSpeed;
			}
			port fuelCommandPort {
				out fuelCommand : FuelCommand;
			}
		}
		
		flow sensedSpeedFlow of SensedSpeed 
			from speedometer_b.sensedSpeedPort.sensedSpeed to cruiseController_b.sensedSpeedPort.sensedSpeed;
		
		part speedometer_b : Speedometer {
			port sensedSpeedPort {
				out sensedSpeed : SensedSpeed;
			}
		}
		
		flow fuelCommandFlow of FuelCommand 
			from cruiseController_b.fuelCommandPort.fuelCommand to engine_b.fuelCommandPort.fuelCommand;

		part engine_b : Engine {
			port fuelCommandPort {
				in fuelCommand : FuelCommand;
			}
		}
	}
	
	occurrence cruiseControlInteraction_b : CruiseControlInteraction {
		part :>> driver :>> driver_b {
			port :>> setSpeedPort {
				event driver::setSpeedSent; 
			}
		}
		
		part :>> vehicle :>> vehicle_b {
			part :>> cruiseController :>> cruiseController_b {
				port :>> setSpeedPort {
					event cruiseController::setSpeedReceived;
				}
			}
			part :>> speedometer :>> speedometer_b {
				port :>> sensedSpeedPort {
					event speedometer::sensedSpeedSent;
				}
			}
			part :>> engine :>> engine_b {
				port :>> fuelCommandPort {
					event engine::fuelCommandReceived;
				}
			}
		}
		
		message :>> setSpeedMessage = driverToVehicleInterface.setSpeedFlow;
		message :>> sensedSpeedMessage = vehicle_b.sensedSpeedFlow;
		message :>> fuelCommandMessage = vehicle_b.fuelCommandFlow;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "27_interaction_realization_2.md"
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
        (range (start 5 3) (end 5 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 18) (end 14 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 3) (end 16 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 28) (end 21 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 4) (end 23 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 4) (end 26 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 4) (end 29 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 26) (end 33 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 23) (end 36 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 4) (end 38 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 26) (end 42 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 18) (end 45 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 4) (end 47 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 12) (end 52 795))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 2) (end 53 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 2) (end 59 421))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 33) (end 60 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 28) (end 65 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 23) (end 70 31))
      )
      (diagnostic
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "sysml")
        (range (start 77 2) (end 77 73))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 77 2) (end 77 73))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Interaction Realization-2' {
	private import 'Interaction Example-1'::*;
	
	part driver_b : Driver {
		port setSpeedPort {
			out setSpeed : SetSpeed;
		}
	}
	
	interface driverToVehicleInterface connect driver_b.setSpeedPort to vehicle_b.setSpeedPort {
		flow setSpeedFlow of SetSpeed 
			from driver_b.setSpeedPort.setSpeed to vehicle_b.setSpeedPort.setSpeed;
	}
	
	part vehicle_b : Vehicle {
		port setSpeedPort {
			in setSpeed : SetSpeed;
		}
		
		bind setSpeedPort = cruiseController_b.setSpeedPort;
		
		part cruiseController_b : CruiseController {
			port setSpeedPort {
				in setSpeed : SetSpeed;
			}
			port sensedSpeedPort {
				in sensedSpeed : SensedSpeed;
			}
			port fuelCommandPort {
				out fuelCommand : FuelCommand;
			}
		}
		
		flow sensedSpeedFlow of SensedSpeed 
			from speedometer_b.sensedSpeedPort.sensedSpeed to cruiseController_b.sensedSpeedPort.sensedSpeed;
		
		part speedometer_b : Speedometer {
			port sensedSpeedPort {
				out sensedSpeed : SensedSpeed;
			}
		}
		
		flow fuelCommandFlow of FuelCommand 
			from cruiseController_b.fuelCommandPort.fuelCommand to engine_b.fuelCommandPort.fuelCommand;

		part engine_b : Engine {
			port fuelCommandPort {
				in fuelCommand : FuelCommand;
			}
		}
	}
	
	occurrence cruiseControlInteraction_b : CruiseControlInteraction {
		part :>> driver :>> driver_b {
			port :>> setSpeedPort {
				event driver::setSpeedSent; 
			}
		}
		
		part :>> vehicle :>> vehicle_b {
			part :>> cruiseController :>> cruiseController_b {
				port :>> setSpeedPort {
					event cruiseController::setSpeedReceived;
				}
			}
			part :>> speedometer :>> speedometer_b {
				port :>> sensedSpeedPort {
					event speedometer::sensedSpeedSent;
				}
			}
			part :>> engine :>> engine_b {
				port :>> fuelCommandPort {
					event engine::fuelCommandReceived;
				}
			}
		}
		
		message :>> setSpeedMessage = driverToVehicleInterface.setSpeedFlow;
		message :>> sensedSpeedMessage = vehicle_b.sensedSpeedFlow;
		message :>> fuelCommandMessage = vehicle_b.fuelCommandFlow;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "b9ac6f67bba225696cf86516a87f720b44c3bc4a11870212a9082515de926aab") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2"))) (kind "package") (name "Interaction Realization-2") (declared-name "Interaction Realization-2") (range (start (line 0) (character 0)) (end (line 0) (character 2046))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 43))) (parent (node (document "d0") (qualified-name "Interaction Realization-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Interaction Example-1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (kind "occurrence") (name "cruiseControlInteraction_b") (declared-name "cruiseControlInteraction_b") (range (start (line 52) (character 12)) (end (line 52) (character 795))) (parent (node (document "d0") (qualified-name "Interaction Realization-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseControlInteraction") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (kind "part") (name "driver") (declared-name "driver") (range (start (line 53) (character 2)) (end (line 53) (character 101))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (range (start (line 54) (character 3)) (end (line 54) (character 64))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "setSpeedPort") (range (start (line 54) (character 12)) (end (line 54) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 59) (character 2)) (end (line 59) (character 421))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (kind "part") (name "cruiseController") (declared-name "cruiseController") (range (start (line 60) (character 3)) (end (line 60) (character 139))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cruiseController_b") (range (start (line 60) (character 33)) (end (line 60) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (range (start (line 61) (character 4)) (end (line 61) (character 80))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "setSpeedPort") (range (start (line 61) (character 13)) (end (line 61) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 70) (character 3)) (end (line 70) (character 115))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine_b") (range (start (line 70) (character 23)) (end (line 70) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (kind "port") (name "fuelCommandPort") (declared-name "fuelCommandPort") (range (start (line 71) (character 4)) (end (line 71) (character 76))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCommandPort") (range (start (line 71) (character 13)) (end (line 71) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (kind "part") (name "speedometer") (declared-name "speedometer") (range (start (line 65) (character 3)) (end (line 65) (character 126))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "speedometer_b") (range (start (line 65) (character 28)) (end (line 65) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (kind "port") (name "sensedSpeedPort") (declared-name "sensedSpeedPort") (range (start (line 66) (character 4)) (end (line 66) (character 77))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sensedSpeedPort") (range (start (line 66) (character 13)) (end (line 66) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::driverToVehicleInterface"))) (kind "kermlDecl") (name "driverToVehicleInterface") (declared-name "driverToVehicleInterface") (range (start (line 9) (character 1)) (end (line 9) (character 204))) (parent (node (document "d0") (qualified-name "Interaction Realization-2"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::driver_b"))) (kind "part") (name "driver_b") (declared-name "driver_b") (range (start (line 3) (character 1)) (end (line 3) (character 82))) (parent (node (document "d0") (qualified-name "Interaction Realization-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driver") (range (start (line 3) (character 17)) (end (line 3) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (range (start (line 4) (character 2)) (end (line 4) (character 53))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::driver_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort::setSpeed"))) (kind "in out parameter") (name "setSpeed") (declared-name "setSpeed") (range (start (line 5) (character 3)) (end (line 5) (character 27))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort"))) (authored (relationships (typing (reference "SetSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind "part") (name "vehicle_b") (declared-name "vehicle_b") (range (start (line 14) (character 1)) (end (line 14) (character 870))) (parent (node (document "d0") (qualified-name "Interaction Realization-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 14) (character 18)) (end (line 14) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind "part") (name "cruiseController_b") (declared-name "cruiseController_b") (range (start (line 21) (character 2)) (end (line 21) (character 237))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseController") (range (start (line 21) (character 28)) (end (line 21) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort"))) (kind "port") (name "fuelCommandPort") (declared-name "fuelCommandPort") (range (start (line 28) (character 3)) (end (line 28) (character 65))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (kind "in out parameter") (name "fuelCommand") (declared-name "fuelCommand") (range (start (line 29) (character 4)) (end (line 29) (character 34))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort"))) (authored (relationships (typing (reference "FuelCommand") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort"))) (kind "port") (name "sensedSpeedPort") (declared-name "sensedSpeedPort") (range (start (line 25) (character 3)) (end (line 25) (character 64))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (kind "in out parameter") (name "sensedSpeed") (declared-name "sensedSpeed") (range (start (line 26) (character 4)) (end (line 26) (character 33))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort"))) (authored (relationships (typing (reference "SensedSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (range (start (line 22) (character 3)) (end (line 22) (character 55))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort::setSpeed"))) (kind "in out parameter") (name "setSpeed") (declared-name "setSpeed") (range (start (line 23) (character 4)) (end (line 23) (character 27))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (authored (relationships (typing (reference "SetSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind "part") (name "engine_b") (declared-name "engine_b") (range (start (line 45) (character 2)) (end (line 45) (character 95))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 45) (character 18)) (end (line 45) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort"))) (kind "port") (name "fuelCommandPort") (declared-name "fuelCommandPort") (range (start (line 46) (character 3)) (end (line 46) (character 64))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (kind "in out parameter") (name "fuelCommand") (declared-name "fuelCommand") (range (start (line 47) (character 4)) (end (line 47) (character 33))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort"))) (authored (relationships (typing (reference "FuelCommand") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow"))) (kind "flow") (name "fuelCommandFlow") (declared-name "fuelCommandFlow") (range (start (line 42) (character 2)) (end (line 42) (character 134))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 42) (character 26)) (end (line 42) (character 37))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow"))) (authored (relationships (typing (reference "FuelCommand") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow"))) (kind "flow") (name "sensedSpeedFlow") (declared-name "sensedSpeedFlow") (range (start (line 33) (character 2)) (end (line 33) (character 139))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 33) (character 26)) (end (line 33) (character 37))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow"))) (authored (relationships (typing (reference "SensedSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (range (start (line 15) (character 2)) (end (line 15) (character 52))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort::setSpeed"))) (kind "in out parameter") (name "setSpeed") (declared-name "setSpeed") (range (start (line 16) (character 3)) (end (line 16) (character 26))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (authored (relationships (typing (reference "SetSpeed") (range none)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind "part") (name "speedometer_b") (declared-name "speedometer_b") (range (start (line 36) (character 2)) (end (line 36) (character 106))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "Speedometer") (range (start (line 36) (character 23)) (end (line 36) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort"))) (kind "port") (name "sensedSpeedPort") (declared-name "sensedSpeedPort") (range (start (line 37) (character 3)) (end (line 37) (character 65))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (kind "in out parameter") (name "sensedSpeed") (declared-name "sensedSpeed") (range (start (line 38) (character 4)) (end (line 38) (character 34))) (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort"))) (authored (relationships (typing (reference "SensedSpeed") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Interaction Example-1::*") (range (start (line 1) (character 16)) (end (line 1) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseControlInteraction") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (kind redefinition) (ordinal 0)) (authored-target "setSpeedPort") (range (start (line 54) (character 12)) (end (line 54) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (kind redefinition) (ordinal 0)) (authored-target "cruiseController_b") (range (start (line 60) (character 33)) (end (line 60) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (kind redefinition) (ordinal 0)) (authored-target "setSpeedPort") (range (start (line 61) (character 13)) (end (line 61) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine_b") (range (start (line 70) (character 23)) (end (line 70) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCommandPort") (range (start (line 71) (character 13)) (end (line 71) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (kind redefinition) (ordinal 0)) (authored-target "speedometer_b") (range (start (line 65) (character 28)) (end (line 65) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (kind redefinition) (ordinal 0)) (authored-target "sensedSpeedPort") (range (start (line 66) (character 13)) (end (line 66) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::driver_b"))) (kind featureTyping) (ordinal 0)) (authored-target "Driver") (range (start (line 3) (character 17)) (end (line 3) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 14) (character 18)) (end (line 14) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind bindSource) (ordinal 0)) (authored-target "setSpeedPort") (range (start (line 19) (character 7)) (end (line 19) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind bindTarget) (ordinal 0)) (authored-target "cruiseController_b::setSpeedPort") (range (start (line 19) (character 22)) (end (line 19) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowSource) (ordinal 1)) (authored-target "speedometer_b::sensedSpeedPort::sensedSpeed") (range (start (line 34) (character 8)) (end (line 34) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowSource) (ordinal 2)) (authored-target "cruiseController_b::fuelCommandPort::fuelCommand") (range (start (line 43) (character 8)) (end (line 43) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowTarget) (ordinal 1)) (authored-target "cruiseController_b::sensedSpeedPort::sensedSpeed") (range (start (line 34) (character 53)) (end (line 34) (character 99))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowTarget) (ordinal 2)) (authored-target "engine_b::fuelCommandPort::fuelCommand") (range (start (line 43) (character 58)) (end (line 43) (character 94))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseController") (range (start (line 21) (character 28)) (end (line 21) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCommand") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SensedSpeed") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 45) (character 18)) (end (line 45) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCommand") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCommand") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "SensedSpeed") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind featureTyping) (ordinal 0)) (authored-target "Speedometer") (range (start (line 36) (character 23)) (end (line 36) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SensedSpeed") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "cruiseController_b::fuelCommandPort::fuelCommand") (target "engine_b::fuelCommandPort::fuelCommand") (source-range (start (line 43) (character 8)) (end (line 43) (character 54))) (target-range (start (line 43) (character 58)) (end (line 43) (character 94)))))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "setSpeedPort") (target "cruiseController_b::setSpeedPort") (source-range (start (line 19) (character 7)) (end (line 19) (character 19))) (target-range (start (line 19) (character 22)) (end (line 19) (character 53)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "speedometer_b::sensedSpeedPort::sensedSpeed") (target "cruiseController_b::sensedSpeedPort::sensedSpeed") (source-range (start (line 34) (character 8)) (end (line 34) (character 49))) (target-range (start (line 34) (character 53)) (end (line 34) (character 99)))))
  )
  (evaluation
  )
)
~~~
