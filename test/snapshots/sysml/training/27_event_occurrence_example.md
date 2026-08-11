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
  (document "27_event_occurrence_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e1a719d5569130159365d8fe1a571ca173b828b7a978c9005c0879f93d4b7972") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example"))) (kind "package") (name "Event Occurrence Example") (declared-name "Event Occurrence Example") (range (start (line 0) (character 0)) (end (line 0) (character 573))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController"))) (kind "part def") (name "CruiseController") (declared-name "CruiseController") (range (start (line 2) (character 1)) (end (line 2) (character 27))) (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::Driver"))) (kind "part def") (name "Driver") (declared-name "Driver") (range (start (line 1) (character 1)) (end (line 1) (character 17))) (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 4) (character 1)) (end (line 4) (character 17))) (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::Speedometer"))) (kind "part def") (name "Speedometer") (declared-name "Speedometer") (range (start (line 3) (character 1)) (end (line 3) (character 22))) (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 5) (character 1)) (end (line 5) (character 18))) (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::driver"))) (kind "part") (name "driver") (declared-name "driver") (range (start (line 7) (character 1)) (end (line 7) (character 59))) (parent (node (document "d0") (qualified-name "Event Occurrence Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driver") (range (start (line 7) (character 15)) (end (line 7) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::driver::setSpeedSent"))) (kind "occurrence") (name "setSpeedSent") (declared-name "setSpeedSent") (range (start (line 8) (character 19)) (end (line 8) (character 32))) (parent (node (document "d0") (qualified-name "Event Occurrence Example::driver"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 11) (character 1)) (end (line 11) (character 363))) (parent (node (document "d0") (qualified-name "Event Occurrence Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 11) (character 16)) (end (line 11) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (kind "part") (name "cruiseController") (declared-name "cruiseController") (range (start (line 13) (character 2)) (end (line 13) (character 178))) (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseController") (range (start (line 13) (character 26)) (end (line 13) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::fuelCommandSent"))) (kind "occurrence") (name "fuelCommandSent") (declared-name "fuelCommandSent") (range (start (line 16) (character 25)) (end (line 16) (character 41))) (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::sensedSpeedReceived"))) (kind "occurrence") (name "sensedSpeedReceived") (declared-name "sensedSpeedReceived") (range (start (line 15) (character 25)) (end (line 15) (character 45))) (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::setSpeedReceived"))) (kind "occurrence") (name "setSpeedReceived") (declared-name "setSpeedReceived") (range (start (line 14) (character 20)) (end (line 14) (character 37))) (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 23) (character 2)) (end (line 23) (character 69))) (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 23) (character 16)) (end (line 23) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine::fuelCommandReceived"))) (kind "occurrence") (name "fuelCommandReceived") (declared-name "fuelCommandReceived") (range (start (line 24) (character 20)) (end (line 24) (character 40))) (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (kind "part") (name "speedometer") (declared-name "speedometer") (range (start (line 19) (character 2)) (end (line 19) (character 75))) (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Speedometer") (range (start (line 19) (character 21)) (end (line 19) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer::sensedSpeedSent"))) (kind "occurrence") (name "sensedSpeedSent") (declared-name "sensedSpeedSent") (range (start (line 20) (character 20)) (end (line 20) (character 36))) (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Driver") (range (start (line 7) (character 15)) (end (line 7) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::Driver")))))
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 11) (character 16)) (end (line 11) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseController") (range (start (line 13) (character 26)) (end (line 13) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController")))))
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 23) (character 16)) (end (line 23) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (kind featureTyping) (ordinal 0)) (authored-target "Speedometer") (range (start (line 19) (character 21)) (end (line 19) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::Speedometer")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Event Occurrence Example::driver"))) (target (node (document "d0") (qualified-name "Event Occurrence Example::Driver"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Event Occurrence Example::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (target (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (target (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))) (target (node (document "d0") (qualified-name "Event Occurrence Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (target (node (document "d0") (qualified-name "Event Occurrence Example::Speedometer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
