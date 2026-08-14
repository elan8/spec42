# META
~~~ini
description=SysML Training 27 (Occurrences): Event Occurrence Example
type=file
~~~
# SOURCE
~~~sysml
package 'Event Occurrence Example' {	
	part def Driver;
	part def CruiseController;
	part def Speedometer;
	part def Engine;
	part def Vehicle;
	
	part driver : Driver {
		event occurrence setSpeedSent;
	}
	
	part vehicle : Vehicle {
	
		part cruiseController : CruiseController {
			event occurrence setSpeedReceived;		
			then event occurrence sensedSpeedReceived;		
			then event occurrence fuelCommandSent;
		}
		
		part speedometer : Speedometer {
			event occurrence sensedSpeedSent;
		}
		
		part engine : Engine {
			event occurrence fuelCommandReceived;
		}
	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/27_event_occurrence_example.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:da535832be0b11f483e1f8407c97f16accae2b8c4bfc9d742e736ce9b64407ae") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::CruiseController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Driver"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Speedometer"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::driver"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driver")))))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::driver::setSpeedSent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CruiseController")))))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::cruiseController::fuelCommandSent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::cruiseController::sensedSpeedReceived"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::cruiseController::setSpeedReceived"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::engine::fuelCommandReceived"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Speedometer")))))
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::speedometer::sensedSpeedSent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Driver")))))
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (kind featureTyping) (ordinal 0))
      (authored-target "CruiseController")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::CruiseController")))))
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Engine")))))
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Speedometer")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Speedometer")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::driver"))) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle"))) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::CruiseController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::engine"))) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Speedometer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::driver")))
      (supertype (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Driver")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle")))
      (supertype (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::cruiseController")))
      (supertype (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::CruiseController")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::engine")))
      (supertype (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::speedometer")))
      (supertype (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Speedometer")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_event_occurrence_example.md") (range (start 7 15) (end 7 21)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Driver")))))
    )
  )
  (query (document "memory://snapshot/27_event_occurrence_example.md") (range (start 11 16) (end 11 23)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/27_event_occurrence_example.md") (range (start 13 26) (end 13 42)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::CruiseController")))))
    )
  )
  (query (document "memory://snapshot/27_event_occurrence_example.md") (range (start 23 16) (end 23 22)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Engine")))))
    )
  )
  (query (document "memory://snapshot/27_event_occurrence_example.md") (range (start 19 21) (end 19 32)) (probe (position 19 21))
    (reference (id (source (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (kind featureTyping) (ordinal 0) (authored-target "Speedometer")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_event_occurrence_example.md") (qualified-name "Event Occurrence Example::Speedometer")))))
    )
  )
)
~~~
