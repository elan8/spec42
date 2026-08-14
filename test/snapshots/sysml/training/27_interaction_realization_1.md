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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 32) (end 5 40))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 41) (end 14 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 23) (end 18 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 36) (end 20 47))
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
        (range (start 31 41) (end 31 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 44) (end 33 56))
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
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Interaction Example-1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseControlInteraction"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver_a"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "driver")) (anonymous (kind occurrence) (ordinal 0)))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "setSpeedSent"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicle_a"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cruiseController_a"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 0)))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "setSpeedReceived"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 1)))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "sensedSpeedReceived"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 2)))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelCommandSent"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine_a"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "engine")) (anonymous (kind occurrence) (ordinal 0)))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelCommandReceived"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "speedometer_a"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "speedometer")) (anonymous (kind occurrence) (ordinal 0)))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "sensedSpeedSent"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driver"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (invocationCallee (reference "SetSpeed")) (sendTarget (reference "vehicle_a"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseController"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (invocationCallee (reference "FuelCommand")) (sendTarget (reference "engine_a"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Speedometer"))))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (invocationCallee (reference "SensedSpeed")) (sendTarget (reference "cruiseController_a"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Interaction Example-1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseControlInteraction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (kind redefinition) (ordinal 0))
      (authored-target "driver_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "driver")) (anonymous (kind occurrence) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "setSpeedSent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (kind redefinition) (ordinal 0))
      (authored-target "vehicle_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (kind redefinition) (ordinal 0))
      (authored-target "cruiseController_a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "setSpeedReceived")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "sensedSpeedReceived")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 2)))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelCommandSent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (kind redefinition) (ordinal 0))
      (authored-target "engine_a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "engine")) (anonymous (kind occurrence) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelCommandReceived")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (kind redefinition) (ordinal 0))
      (authored-target "speedometer_a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "speedometer")) (anonymous (kind occurrence) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "sensedSpeedSent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (kind invocationCallee) (ordinal 0))
      (authored-target "SetSpeed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (kind sendTarget) (ordinal 0))
      (authored-target "vehicle_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseController")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind invocationCallee) (ordinal 0))
      (authored-target "FuelCommand")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind sendTarget) (ordinal 0))
      (authored-target "engine_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Speedometer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (kind invocationCallee) (ordinal 0))
      (authored-target "SensedSpeed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (kind sendTarget) (ordinal 0))
      (authored-target "cruiseController_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind sendTarget) (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (kind sendTarget) (ordinal 0)))
    (relationship (kind sendTarget) (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind sendTarget) (ordinal 0)))
    (relationship (kind sendTarget) (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (kind sendTarget) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver")))
      (supertype (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle")))
      (supertype (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 1 16) (end 1 42)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Interaction Example-1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 31 41) (end 31 65)) (probe (position 31 41))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseControlInteraction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 32 22) (end 32 30)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (kind redefinition) (ordinal 0) (authored-target "driver_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a")))))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 33 44) (end 33 56)) (probe (position 33 44))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "driver")) (anonymous (kind occurrence) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "setSpeedSent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 36 23) (end 36 32)) (probe (position 36 23))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (kind redefinition) (ordinal 0) (authored-target "vehicle_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a")))))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 37 33) (end 37 51)) (probe (position 37 33))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (kind redefinition) (ordinal 0) (authored-target "cruiseController_a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 38 52) (end 38 68)) (probe (position 38 52))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "setSpeedReceived")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 39 55) (end 39 74)) (probe (position 39 55))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "sensedSpeedReceived")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 40 52) (end 40 67)) (probe (position 40 52))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 2)))))) (kind redefinition) (ordinal 0) (authored-target "fuelCommandSent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 45 23) (end 45 31)) (probe (position 45 23))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (kind redefinition) (ordinal 0) (authored-target "engine_a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 46 51) (end 46 70)) (probe (position 46 51))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "engine")) (anonymous (kind occurrence) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "fuelCommandReceived")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 42 28) (end 42 41)) (probe (position 42 28))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (kind redefinition) (ordinal 0) (authored-target "speedometer_a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 43 53) (end 43 68)) (probe (position 43 53))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (path (named (kind package) (name "Interaction Realization-1")) (named (kind occurrence) (name "cruiseControlInteraction_a")) (named (kind part) (name "vehicle")) (named (kind part) (name "speedometer")) (anonymous (kind occurrence) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "sensedSpeedSent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 3 17) (end 3 23)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a"))) (kind featureTyping) (ordinal 0) (authored-target "Driver")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 5 32) (end 5 40)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (kind invocationCallee) (ordinal 0) (authored-target "SetSpeed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 5 46) (end 5 55)) (probe (position 5 46))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (kind sendTarget) (ordinal 0) (authored-target "vehicle_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a")))))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 9 18) (end 9 25)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 10 28) (end 10 44)) (probe (position 10 28))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 14 41) (end 14 52)) (probe (position 14 41))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind invocationCallee) (ordinal 0) (authored-target "FuelCommand")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 14 58) (end 14 66)) (probe (position 14 58))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (kind sendTarget) (ordinal 0) (authored-target "engine_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a")))))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 24 18) (end 24 24)) (probe (position 24 18))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 18 23) (end 18 34)) (probe (position 18 23))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (kind featureTyping) (ordinal 0) (authored-target "Speedometer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 20 36) (end 20 47)) (probe (position 20 36))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (kind invocationCallee) (ordinal 0) (authored-target "SensedSpeed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_interaction_realization_1.md") (range (start 20 53) (end 20 71)) (probe (position 20 53))
    (reference (id (source (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (kind sendTarget) (ordinal 0) (authored-target "cruiseController_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_realization_1.md") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a")))))
  )
)
~~~
