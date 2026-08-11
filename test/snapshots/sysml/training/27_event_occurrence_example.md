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
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example"))) (kind "package") (name "Event Occurrence Example") (declared-name "Event Occurrence Example"))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController"))) (kind "part def") (name "CruiseController") (declared-name "CruiseController") (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::Driver"))) (kind "part def") (name "Driver") (declared-name "Driver") (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::Speedometer"))) (kind "part def") (name "Speedometer") (declared-name "Speedometer") (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Event Occurrence Example"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::driver"))) (kind "part") (name "driver") (declared-name "driver") (parent (node (document "d0") (qualified-name "Event Occurrence Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driver")))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::driver::setSpeedSent"))) (kind "occurrence") (name "setSpeedSent") (declared-name "setSpeedSent") (parent (node (document "d0") (qualified-name "Event Occurrence Example::driver"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Event Occurrence Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (kind "part") (name "cruiseController") (declared-name "cruiseController") (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "CruiseController")))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::fuelCommandSent"))) (kind "occurrence") (name "fuelCommandSent") (declared-name "fuelCommandSent") (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::sensedSpeedReceived"))) (kind "occurrence") (name "sensedSpeedReceived") (declared-name "sensedSpeedReceived") (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::setSpeedReceived"))) (kind "occurrence") (name "setSpeedReceived") (declared-name "setSpeedReceived") (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine::fuelCommandReceived"))) (kind "occurrence") (name "fuelCommandReceived") (declared-name "fuelCommandReceived") (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (kind "part") (name "speedometer") (declared-name "speedometer") (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Speedometer")))))
    (element (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer::sensedSpeedSent"))) (kind "occurrence") (name "sensedSpeedSent") (declared-name "sensedSpeedSent") (parent (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Driver") (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::Driver")))))
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (kind featureTyping) (ordinal 0)) (authored-target "CruiseController") (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController")))))
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (kind featureTyping) (ordinal 0)) (authored-target "Speedometer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Event Occurrence Example::Speedometer")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 15) (end 7 21)) (probe (position 7 15))
      (reference
        (source (document "d0") (qualified-name "Event Occurrence Example::driver"))
        (kind featureTyping) (ordinal 0) (authored-target "Driver")
        (range (start 7 15) (end 7 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Event Occurrence Example::Driver") (range (start 1 1) (end 1 17)))
        )
      )
    )
    (query (range (start 23 16) (end 23 22)) (probe (position 23 16))
      (reference
        (source (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 23 16) (end 23 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Event Occurrence Example::Engine") (range (start 4 1) (end 4 17)))
        )
      )
    )
    (query (range (start 11 16) (end 11 23)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Event Occurrence Example::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 11 16) (end 11 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Event Occurrence Example::Vehicle") (range (start 5 1) (end 5 18)))
        )
      )
    )
    (query (range (start 19 21) (end 19 32)) (probe (position 19 21))
      (reference
        (source (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))
        (kind featureTyping) (ordinal 0) (authored-target "Speedometer")
        (range (start 19 21) (end 19 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Event Occurrence Example::Speedometer") (range (start 3 1) (end 3 22)))
        )
      )
    )
    (query (range (start 13 26) (end 13 42)) (probe (position 13 26))
      (reference
        (source (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))
        (kind featureTyping) (ordinal 0) (authored-target "CruiseController")
        (range (start 13 26) (end 13 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Event Occurrence Example::CruiseController") (range (start 2 1) (end 2 27)))
        )
      )
    )
  )
)
~~~
