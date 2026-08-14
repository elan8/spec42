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
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 19 2) (end 19 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 20 2) (end 20 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 22 2) (end 23 73))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:d260b826265ab857599c63a4b4eba9dbb18c5a3f20a1d3be4f8e49cb5de2625b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Event Occurrence Example") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::CruiseControlInteraction"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand::fuelFlow"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real"))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::SensedSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::SetSpeed"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EngineController"))))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandForwarded"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController::fuelCommandReceived"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
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
    (relationship (kind typing) (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 1 16) (end 1 45)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (path (named (kind package) (name "Message Payload Example")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Event Occurrence Example")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 6 23) (end 6 41)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::FuelCommand::fuelFlow"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 11 18) (end 11 25)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/27_message_payload_example.md") (range (start 12 26) (end 12 42)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::vehicle1::engineController"))) (kind featureTyping) (ordinal 0) (authored-target "EngineController")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_message_payload_example.md") (qualified-name "Message Payload Example::EngineController")))))
  )
)
~~~
