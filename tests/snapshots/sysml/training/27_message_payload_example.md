# META
~~~ini
description=SysML Training 27 (Occurrences): Message Payload Example
type=file
~~~
# SOURCE
~~~sysml
package 'Message Payload Example' {
	private import 'Event Occurrence Example'::*;
	
	item def SetSpeed;
	item def SensedSpeed;
	item def FuelCommand {
		attribute fuelFlow : ScalarValues::Real;
	}
	
	part def EngineController;
	
	part vehicle1 :> vehicle {
		part engineController : EngineController {
			event occurrence fuelCommandReceived;
			then event occurrence fuelCommandForwarded;
		}
	}
	
	occurrence def CruiseControlInteraction {		
		ref part :>> driver;		
		ref part vehicle :>> vehicle1;
		
		message setSpeedMessage of SetSpeed 
			from driver.setSpeedSent to vehicle.cruiseController.setSpeedReceived;
			
		then message sensedSpeedMessage of SensedSpeed 
			from vehicle.speedometer.sensedSpeedSent to vehicle.cruiseController.sensedSpeedReceived;
			
		then message fuelCommandMessage of fuelCommand : FuelCommand 
			from vehicle.cruiseController.fuelCommandSent to vehicle.engineController.fuelCommandReceived;
		
		then message fuelCommandForwardingMessage of fuelCommand : FuelCommand = fuelCommandMessage.fuelCommand
			from vehicle.engineController.fuelCommandForwarded to vehicle.engine.fuelCommandReceived;
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/27_message_payload_example.md"
    (diagnostics
      (diagnostic
        (severity information)
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
        (range (start 6 23) (end 6 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 18) (end 11 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 15) (end 19 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 8) (end 23 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 31) (end 23 72))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 25 2) (end 28 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 28 2) (end 31 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 31 2) (end 34 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:d260b826265ab857599c63a4b4eba9dbb18c5a3f20a1d3be4f8e49cb5de2625b") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Event Occurrence Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver")))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "driver::setSpeedSent")) (flowTarget (reference "vehicle::cruiseController::setSpeedReceived")) (flowPayloadType (reference "SetSpeed")))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicle1")))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand::fuelFlow"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::SensedSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::SetSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EngineController")))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandForwarded"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandReceived"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (kind flowSource) (ordinal 0))
      (authored-target "driver::setSpeedSent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (kind flowTarget) (ordinal 0))
      (authored-target "vehicle::cruiseController::setSpeedReceived")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "SetSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::SetSpeed")))))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::vehicle"))) (kind redefinition) (ordinal 0))
      (authored-target "vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1")))))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand::fuelFlow"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind featureTyping) (ordinal 0))
      (authored-target "EngineController")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController")))))
  )
  (relationships
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::SetSpeed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::vehicle"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::vehicle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::vehicle"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand::fuelFlow"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandForwarded"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandReceived"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage")))
      (featured-by (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction")))
    )
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::vehicle")))
      (featured-by (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction")))
      (supertype (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController")))
      (subtype (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand::fuelFlow")))
      (featured-by (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand")))
    )
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1")))
      (subtype (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::vehicle")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController")))
      (featured-by (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1")))
      (type (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController")) (provenance authored))
      (effective-type (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController")) (source direct))
      (supertype (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandForwarded")))
      (featured-by (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController")))
    )
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandReceived")))
      (featured-by (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 1 16) (end 1 45)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 19 15) (end 19 21)) (probe (position 19 15))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (named (kind occurrence-def) (name "CruiseControlInteraction")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 23 8) (end 23 27)) (probe (position 23 8))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (kind flowSource) (ordinal 0) (authored-target "driver::setSpeedSent")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 23 31) (end 23 72)) (probe (position 23 31))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (kind flowTarget) (ordinal 0) (authored-target "vehicle::cruiseController::setSpeedReceived")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 22 29) (end 22 37)) (probe (position 22 29))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::setSpeedMessage"))) (kind flowPayloadType) (ordinal 0) (authored-target "SetSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::SetSpeed")))))
    )
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 20 23) (end 20 31)) (probe (position 20 23))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction::vehicle"))) (kind redefinition) (ordinal 0) (authored-target "vehicle1")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1")))))
    )
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 6 23) (end 6 41)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand::fuelFlow"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 11 18) (end 11 25)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 12 26) (end 12 42)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind featureTyping) (ordinal 0) (authored-target "EngineController")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController")))))
    )
  )
)
~~~
