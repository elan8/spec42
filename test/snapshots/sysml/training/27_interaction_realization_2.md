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
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 5 3) (end 5 27))
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
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 16 3) (end 16 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 19 2) (end 19 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 28) (end 21 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 23 4) (end 23 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 26 4) (end 26 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 29 4) (end 29 34))
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
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 38 4) (end 38 34))
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
        (code "unsupported_port_usage_member")
        (source "semantic")
        (range (start 47 4) (end 47 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 52 12) (end 80 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 55 4) (end 56 3))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 62 5) (end 63 4))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 67 5) (end 68 4))
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
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driver"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseController"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Speedometer"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort"))) (kind port) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Interaction Example-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseController")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind featureTyping) (ordinal 0))
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
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 1 16) (end 1 42)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Interaction Example-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 3 17) (end 3 23)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::driver_b"))) (kind featureTyping) (ordinal 0) (authored-target "Driver")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 14 18) (end 14 25)) (probe (position 14 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 21 28) (end 21 44)) (probe (position 21 28))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 45 18) (end 45 24)) (probe (position 45 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_2.md") (range (start 36 23) (end 36 34)) (probe (position 36 23))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_2.md") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (kind featureTyping) (ordinal 0) (authored-target "Speedometer")
      (outcome (status unresolved)))
  )
)
~~~
