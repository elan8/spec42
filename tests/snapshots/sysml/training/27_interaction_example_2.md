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
  (document "memory://snapshot/27_interaction_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 20) (end 9 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 21) (end 13 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 27) (end 14 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 22) (end 20 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 17) (end 24 23))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 30 2) (end 31 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:a7d64d59aa843b02caf516aad5856077fb1fec8581e6cedd28cdcf6c23048bec"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Event Occurrence Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::driver"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driver")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "driver")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowPayloadType (reference "FuelCommand")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowPayloadType (reference "SetSpeed")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseController")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 2))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "engine")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::speedometer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Speedometer")))))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "speedometer")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::FuelCommand"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::SensedSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::SetSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "FuelCommand")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::FuelCommand")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "SetSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::SetSpeed")))))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseController")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::speedometer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Speedometer")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage"))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::FuelCommand"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage"))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::SetSpeed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::driver"))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "driver")) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::driver"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage"))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage"))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle"))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController"))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 1))))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 2))))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::engine"))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "engine")) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::speedometer"))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "speedometer")) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::speedometer"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::driver")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "driver")) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::driver")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "cruiseController")) (anonymous (kind occurrence) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::engine")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "engine")) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::engine")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::speedometer")))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (named (kind part) (name "vehicle")) (named (kind part) (name "speedometer")) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::speedometer")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_interaction_example_2.md") (range (start 1 16) (end 1 45)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (path (named (kind package) (name "Interaction Example-2")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_2.md") (range (start 9 20) (end 9 26)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Driver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_2.md") (range (start 31 32) (end 31 43)) (probe (position 31 32))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::fuelCommandMessage"))) (kind flowPayloadType) (ordinal 0) (authored-target "FuelCommand")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::FuelCommand")))))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_2.md") (range (start 29 29) (end 29 37)) (probe (position 29 29))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::setSpeedMessage"))) (kind flowPayloadType) (ordinal 0) (authored-target "SetSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::SetSpeed")))))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_2.md") (range (start 13 21) (end 13 28)) (probe (position 13 21))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_2.md") (range (start 14 27) (end 14 43)) (probe (position 14 27))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::cruiseController"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_2.md") (range (start 24 17) (end 24 23)) (probe (position 24 17))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_interaction_example_2.md") (range (start 20 22) (end 20 33)) (probe (position 20 22))
    (reference (id (source (node (document "memory://snapshot/27_interaction_example_2.md") (qualified-name "Interaction Example-2::CruiseControlInteraction::vehicle::speedometer"))) (kind featureTyping) (ordinal 0) (authored-target "Speedometer")
      (outcome (status unresolved)))
    )
  )
)
~~~
