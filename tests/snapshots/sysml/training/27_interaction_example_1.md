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
  (document "memory://snapshot/27_interaction_example_1.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 15) (end 1 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 15) (end 8 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 15) (end 9 22))
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
        (source "parser")
        (range (start 20 2) (end 21 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:cf0e9b65263b27844b51d637865df9ce30063a7a0957b0425da2a04c63b8ff91") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Event Occurrence Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "vehicle::cruiseController::fuelCommandSent")) (flowTarget (reference "vehicle::engine::fuelCommandReceived")) (flowPayloadType (reference "FuelCommand")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "vehicle::speedometer::sensedSpeedSent")) (flowTarget (reference "vehicle::cruiseController::sensedSpeedReceived")) (flowPayloadType (reference "SensedSpeed")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "driver::setSpeedSent")) (flowTarget (reference "vehicle::cruiseController::setSpeedReceived")) (flowPayloadType (reference "SetSpeed")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::FuelCommand"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::SensedSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::SetSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind flowSource) (ordinal 0))
      (authored-target "vehicle::cruiseController::fuelCommandSent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind flowTarget) (ordinal 0))
      (authored-target "vehicle::engine::fuelCommandReceived")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "FuelCommand")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::FuelCommand")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind flowSource) (ordinal 0))
      (authored-target "vehicle::speedometer::sensedSpeedSent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind flowTarget) (ordinal 0))
      (authored-target "vehicle::cruiseController::sensedSpeedReceived")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "SensedSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::SensedSpeed")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind flowSource) (ordinal 0))
      (authored-target "driver::setSpeedSent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind flowTarget) (ordinal 0))
      (authored-target "vehicle::cruiseController::setSpeedReceived")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "SetSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::SetSpeed")))))
  )
  (relationships
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::FuelCommand"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::SensedSpeed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::SetSpeed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 1))))) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 1 15) (end 1 44)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 8 15) (end 8 21)) (probe (position 8 15))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 9 15) (end 9 22)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (path (named (kind package) (name "Interaction Example-1")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 18 8) (end 18 48)) (probe (position 18 8))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind flowSource) (ordinal 0) (authored-target "vehicle::cruiseController::fuelCommandSent")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 18 52) (end 18 86)) (probe (position 18 52))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind flowTarget) (ordinal 0) (authored-target "vehicle::engine::fuelCommandReceived")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 17 32) (end 17 43)) (probe (position 17 32))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::fuelCommandMessage"))) (kind flowPayloadType) (ordinal 0) (authored-target "FuelCommand")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::FuelCommand")))))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 15 8) (end 15 43)) (probe (position 15 8))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind flowSource) (ordinal 0) (authored-target "vehicle::speedometer::sensedSpeedSent")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 15 47) (end 15 91)) (probe (position 15 47))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind flowTarget) (ordinal 0) (authored-target "vehicle::cruiseController::sensedSpeedReceived")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 14 32) (end 14 43)) (probe (position 14 32))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::sensedSpeedMessage"))) (kind flowPayloadType) (ordinal 0) (authored-target "SensedSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::SensedSpeed")))))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 12 8) (end 12 27)) (probe (position 12 8))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind flowSource) (ordinal 0) (authored-target "driver::setSpeedSent")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 12 31) (end 12 72)) (probe (position 12 31))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind flowTarget) (ordinal 0) (authored-target "vehicle::cruiseController::setSpeedReceived")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_1.md") (range (start 11 29) (end 11 37)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::CruiseControlInteraction::setSpeedMessage"))) (kind flowPayloadType) (ordinal 0) (authored-target "SetSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_1.md") (qualified-name "Interaction Example-1::SetSpeed")))))
    )
  )
)
~~~
