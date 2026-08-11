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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "b9ac6f67bba225696cf86516a87f720b44c3bc4a11870212a9082515de926aab") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2"))) (kind "package") (name "Interaction Realization-2") (declared-name "Interaction Realization-2"))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Interaction Realization-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Interaction Example-1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (kind "occurrence") (name "cruiseControlInteraction_b") (declared-name "cruiseControlInteraction_b") (parent (node (document "d0") (qualified-name "Interaction Realization-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseControlInteraction")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (kind "part") (name "driver") (declared-name "driver") (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "setSpeedPort")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (kind "part") (name "cruiseController") (declared-name "cruiseController") (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cruiseController_b")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "setSpeedPort")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine_b")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (kind "port") (name "fuelCommandPort") (declared-name "fuelCommandPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCommandPort")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (kind "part") (name "speedometer") (declared-name "speedometer") (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "speedometer_b")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (kind "port") (name "sensedSpeedPort") (declared-name "sensedSpeedPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "sensedSpeedPort")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::driverToVehicleInterface"))) (kind "kermlDecl") (name "driverToVehicleInterface") (declared-name "driverToVehicleInterface") (parent (node (document "d0") (qualified-name "Interaction Realization-2"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::driver_b"))) (kind "part") (name "driver_b") (declared-name "driver_b") (parent (node (document "d0") (qualified-name "Interaction Realization-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driver")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::driver_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort::setSpeed"))) (kind "in out parameter") (name "setSpeed") (declared-name "setSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort"))) (authored (relationships (typing (reference "SetSpeed")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind "part") (name "vehicle_b") (declared-name "vehicle_b") (parent (node (document "d0") (qualified-name "Interaction Realization-2"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind "part") (name "cruiseController_b") (declared-name "cruiseController_b") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseController")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort"))) (kind "port") (name "fuelCommandPort") (declared-name "fuelCommandPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (kind "in out parameter") (name "fuelCommand") (declared-name "fuelCommand") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort"))) (authored (relationships (typing (reference "FuelCommand")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort"))) (kind "port") (name "sensedSpeedPort") (declared-name "sensedSpeedPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (kind "in out parameter") (name "sensedSpeed") (declared-name "sensedSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort"))) (authored (relationships (typing (reference "SensedSpeed")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort::setSpeed"))) (kind "in out parameter") (name "setSpeed") (declared-name "setSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (authored (relationships (typing (reference "SetSpeed")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind "part") (name "engine_b") (declared-name "engine_b") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort"))) (kind "port") (name "fuelCommandPort") (declared-name "fuelCommandPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (kind "in out parameter") (name "fuelCommand") (declared-name "fuelCommand") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort"))) (authored (relationships (typing (reference "FuelCommand")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow"))) (kind "flow") (name "fuelCommandFlow") (declared-name "fuelCommandFlow") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow"))) (authored (relationships (typing (reference "FuelCommand")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow"))) (kind "flow") (name "sensedSpeedFlow") (declared-name "sensedSpeedFlow") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow"))) (authored (relationships (typing (reference "SensedSpeed")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (kind "port") (name "setSpeedPort") (declared-name "setSpeedPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort::setSpeed"))) (kind "in out parameter") (name "setSpeed") (declared-name "setSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (authored (relationships (typing (reference "SetSpeed")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind "part") (name "speedometer_b") (declared-name "speedometer_b") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (authored (membership (kind Feature)) (relationships (typing (reference "Speedometer")))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort"))) (kind "port") (name "sensedSpeedPort") (declared-name "sensedSpeedPort") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))))
    (element (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (kind "in out parameter") (name "sensedSpeed") (declared-name "sensedSpeed") (parent (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort"))) (authored (relationships (typing (reference "SensedSpeed")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Interaction Example-1::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseControlInteraction") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (kind redefinition) (ordinal 0)) (authored-target "setSpeedPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (kind redefinition) (ordinal 0)) (authored-target "cruiseController_b") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (kind redefinition) (ordinal 0)) (authored-target "setSpeedPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine_b") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCommandPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (kind redefinition) (ordinal 0)) (authored-target "speedometer_b") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (kind redefinition) (ordinal 0)) (authored-target "sensedSpeedPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::driver_b"))) (kind featureTyping) (ordinal 0)) (authored-target "Driver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind bindSource) (ordinal 0)) (authored-target "setSpeedPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind bindTarget) (ordinal 0)) (authored-target "cruiseController_b::setSpeedPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowSource) (ordinal 1)) (authored-target "speedometer_b::sensedSpeedPort::sensedSpeed") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowSource) (ordinal 2)) (authored-target "cruiseController_b::fuelCommandPort::fuelCommand") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowTarget) (ordinal 1)) (authored-target "cruiseController_b::sensedSpeedPort::sensedSpeed") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowTarget) (ordinal 2)) (authored-target "engine_b::fuelCommandPort::fuelCommand") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand")))))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseController") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCommand") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SensedSpeed") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCommand") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCommand") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "SensedSpeed") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SetSpeed") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind featureTyping) (ordinal 0)) (authored-target "Speedometer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (kind featureTyping) (ordinal 0)) (authored-target "SensedSpeed") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "cruiseController_b::fuelCommandPort::fuelCommand") (target "engine_b::fuelCommandPort::fuelCommand")))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "setSpeedPort") (target "cruiseController_b::setSpeedPort")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (target (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "speedometer_b::sensedSpeedPort::sensedSpeed") (target "cruiseController_b::sensedSpeedPort::sensedSpeed")))
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
        (source (document "d0") (qualified-name "Interaction Realization-2::driver_b"))
        (kind featureTyping) (ordinal 0) (authored-target "Driver")
        (range (start 3 17) (end 3 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 18) (end 45 24)) (probe (position 45 18))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 45 18) (end 45 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 18) (end 14 25)) (probe (position 14 18))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 14 18) (end 14 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 70 23) (end 70 31)) (probe (position 70 23))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))
        (kind redefinition) (ordinal 0) (authored-target "engine_b")
        (range (start 70 23) (end 70 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 36 23) (end 36 34)) (probe (position 36 23))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))
        (kind featureTyping) (ordinal 0) (authored-target "Speedometer")
        (range (start 36 23) (end 36 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 7) (end 19 19)) (probe (position 19 7))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))
        (kind bindSource) (ordinal 0) (authored-target "setSpeedPort")
        (range (start 19 7) (end 19 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort") (range (start 15 2) (end 15 52)))
        )
      )
    )
    (query (range (start 54 12) (end 54 24)) (probe (position 54 12))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))
        (kind redefinition) (ordinal 0) (authored-target "setSpeedPort")
        (range (start 54 12) (end 54 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort") (range (start 54 3) (end 54 64)))
        )
      )
    )
    (query (range (start 61 13) (end 61 25)) (probe (position 61 13))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))
        (kind redefinition) (ordinal 0) (authored-target "setSpeedPort")
        (range (start 61 13) (end 61 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort") (range (start 61 4) (end 61 80)))
        )
      )
    )
    (query (range (start 65 28) (end 65 41)) (probe (position 65 28))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))
        (kind redefinition) (ordinal 0) (authored-target "speedometer_b")
        (range (start 65 28) (end 65 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 66 13) (end 66 28)) (probe (position 66 13))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))
        (kind redefinition) (ordinal 0) (authored-target "sensedSpeedPort")
        (range (start 66 13) (end 66 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort") (range (start 66 4) (end 66 77)))
        )
      )
    )
    (query (range (start 71 13) (end 71 28)) (probe (position 71 13))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))
        (kind redefinition) (ordinal 0) (authored-target "fuelCommandPort")
        (range (start 71 13) (end 71 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort") (range (start 71 4) (end 71 76)))
        )
      )
    )
    (query (range (start 21 28) (end 21 44)) (probe (position 21 28))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))
        (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
        (range (start 21 28) (end 21 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 60 33) (end 60 51)) (probe (position 60 33))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))
        (kind redefinition) (ordinal 0) (authored-target "cruiseController_b")
        (range (start 60 33) (end 60 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 39)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Interaction Example-1::*")
        (range (start 1 16) (end 1 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 22) (end 19 53)) (probe (position 19 22))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))
        (kind bindTarget) (ordinal 0) (authored-target "cruiseController_b::setSpeedPort")
        (range (start 19 22) (end 19 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort") (range (start 22 3) (end 22 55)))
        )
      )
    )
    (query (range (start 43 58) (end 43 94)) (probe (position 43 58))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))
        (kind flowTarget) (ordinal 2) (authored-target "engine_b::fuelCommandPort::fuelCommand")
        (range (start 43 58) (end 43 94))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand") (range (start 47 4) (end 47 33)))
        )
      )
    )
    (query (range (start 34 8) (end 34 49)) (probe (position 34 8))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))
        (kind flowSource) (ordinal 1) (authored-target "speedometer_b::sensedSpeedPort::sensedSpeed")
        (range (start 34 8) (end 34 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed") (range (start 38 4) (end 38 34)))
        )
      )
    )
    (query (range (start 34 53) (end 34 99)) (probe (position 34 53))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))
        (kind flowTarget) (ordinal 1) (authored-target "cruiseController_b::sensedSpeedPort::sensedSpeed")
        (range (start 34 53) (end 34 99))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed") (range (start 26 4) (end 26 33)))
        )
      )
    )
    (query (range (start 43 8) (end 43 54)) (probe (position 43 8))
      (reference
        (source (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))
        (kind flowSource) (ordinal 2) (authored-target "cruiseController_b::fuelCommandPort::fuelCommand")
        (range (start 43 8) (end 43 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand") (range (start 29 4) (end 29 34)))
        )
      )
    )
  )
)
~~~
