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
  (document "memory://snapshot/27_interaction_realization_2.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 18) (end 5 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 9 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 1) (end 12 2))
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
        (range (start 16 17) (end 16 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 22) (end 19 53))
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
        (range (start 23 18) (end 23 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 21) (end 26 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 22) (end 29 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 33 2) (end 34 100))
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
        (range (start 38 22) (end 38 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 42 2) (end 43 95))
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
        (range (start 47 21) (end 47 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 41) (end 52 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 12) (end 54 24))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 55 4) (end 56 3))
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
        (range (start 61 13) (end 61 25))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 62 5) (end 63 4))
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
        (range (start 66 13) (end 66 28))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 67 5) (end 68 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 23) (end 70 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 71 13) (end 71 28))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 72 5) (end 73 4))
      )
      (diagnostic
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "parser")
        (range (start 77 2) (end 78 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 77 2) (end 78 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:6c511777c66bf2bd8d655e1b31af7a82befea23a8cafe538d6db3ff73da823ea") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Interaction Example-1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseControlInteraction"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver_b"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "setSpeedPort"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicle_b"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cruiseController_b"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "setSpeedPort"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine_b"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelCommandPort"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "speedometer_b"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "sensedSpeedPort"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driver"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort::setSpeed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SetSpeed") (direction out))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "setSpeedPort")) (memberAccessOperand (reference "cruiseController_b::setSpeedPort"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseController"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCommand") (direction out))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SensedSpeed") (direction in))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort::setSpeed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SetSpeed") (direction in))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCommand") (direction in))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort::setSpeed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SetSpeed") (direction in))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Speedometer"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SensedSpeed") (direction out))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Interaction Example-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseControlInteraction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (kind redefinition) (ordinal 0))
      (authored-target "driver_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "setSpeedPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (kind redefinition) (ordinal 0))
      (authored-target "vehicle_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (kind redefinition) (ordinal 0))
      (authored-target "cruiseController_b")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "setSpeedPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (kind redefinition) (ordinal 0))
      (authored-target "engine_b")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelCommandPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (kind redefinition) (ordinal 0))
      (authored-target "speedometer_b")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "sensedSpeedPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0))
      (authored-target "SetSpeed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "setSpeedPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "cruiseController_b::setSpeedPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseController")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCommand")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (kind featureTyping) (ordinal 0))
      (authored-target "SensedSpeed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0))
      (authored-target "SetSpeed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCommand")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0))
      (authored-target "SetSpeed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Speedometer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (kind featureTyping) (ordinal 0))
      (authored-target "SensedSpeed")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (target (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (target (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 1 16) (end 1 42)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Interaction Example-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 52 41) (end 52 65)) (probe (position 52 41))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseControlInteraction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 53 22) (end 53 30)) (probe (position 53 22))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (kind redefinition) (ordinal 0) (authored-target "driver_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b")))))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 54 12) (end 54 24)) (probe (position 54 12))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "setSpeedPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 59 23) (end 59 32)) (probe (position 59 23))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (kind redefinition) (ordinal 0) (authored-target "vehicle_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b")))))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 60 33) (end 60 51)) (probe (position 60 33))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (kind redefinition) (ordinal 0) (authored-target "cruiseController_b")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 61 13) (end 61 25)) (probe (position 61 13))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "setSpeedPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 70 23) (end 70 31)) (probe (position 70 23))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (kind redefinition) (ordinal 0) (authored-target "engine_b")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 71 13) (end 71 28)) (probe (position 71 13))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fuelCommandPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 65 28) (end 65 41)) (probe (position 65 28))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (kind redefinition) (ordinal 0) (authored-target "speedometer_b")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 66 13) (end 66 28)) (probe (position 66 13))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "sensedSpeedPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 3 17) (end 3 23)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b"))) (kind featureTyping) (ordinal 0) (authored-target "Driver")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 5 18) (end 5 26)) (probe (position 5 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0) (authored-target "SetSpeed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 14 18) (end 14 25)) (probe (position 14 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 19 7) (end 19 19)) (probe (position 19 7))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "setSpeedPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort")))))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 19 22) (end 19 53)) (probe (position 19 22))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "cruiseController_b::setSpeedPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 21 28) (end 21 44)) (probe (position 21 28))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 29 22) (end 29 33)) (probe (position 29 22))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCommand")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 26 21) (end 26 32)) (probe (position 26 21))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (kind featureTyping) (ordinal 0) (authored-target "SensedSpeed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 23 18) (end 23 26)) (probe (position 23 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0) (authored-target "SetSpeed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 45 18) (end 45 24)) (probe (position 45 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 47 21) (end 47 32)) (probe (position 47 21))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCommand")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 16 17) (end 16 25)) (probe (position 16 17))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort::setSpeed"))) (kind featureTyping) (ordinal 0) (authored-target "SetSpeed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 36 23) (end 36 34)) (probe (position 36 23))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind featureTyping) (ordinal 0) (authored-target "Speedometer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 38 22) (end 38 33)) (probe (position 38 22))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (kind featureTyping) (ordinal 0) (authored-target "SensedSpeed")
      (outcome (status unresolved)))
  )
)
~~~
