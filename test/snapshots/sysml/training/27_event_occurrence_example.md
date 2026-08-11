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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Event Occurrence Example''
    (part_def 'Driver')
    (part_def 'CruiseController')
    (part_def 'Speedometer')
    (part_def 'Engine')
    (part_def 'Vehicle')
    (part_usage 'driver' : 'Driver'
      (event_occurrence 'setSpeedSent'))
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'cruiseController' : 'CruiseController'
        (event_occurrence 'setSpeedReceived')
        (source_succession
          (event_occurrence 'sensedSpeedReceived'))
        (source_succession
          (event_occurrence 'fuelCommandSent')))
      (part_usage 'speedometer' : 'Speedometer'
        (event_occurrence 'sensedSpeedSent'))
      (part_usage 'engine' : 'Engine'
        (event_occurrence 'fuelCommandReceived')))))
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Event Occurrence Example"))) (name "Event Occurrence Example") (declared-name "Event Occurrence Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController"))) (name "CruiseController") (declared-name "CruiseController") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Event Occurrence Example::Driver"))) (name "Driver") (declared-name "Driver") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Event Occurrence Example::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Event Occurrence Example::Speedometer"))) (name "Speedometer") (declared-name "Speedometer") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Event Occurrence Example::driver"))) (name "driver") (declared-name "driver") (declared (properties (ordered false)))
          (contains
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Event Occurrence Example::driver::setSpeedSent"))) (name "setSpeedSent") (declared-name "setSpeedSent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Event Occurrence Example::Driver")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (name "cruiseController") (declared-name "cruiseController") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::fuelCommandSent"))) (name "fuelCommandSent") (declared-name "fuelCommandSent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::sensedSpeedReceived"))) (name "sensedSpeedReceived") (declared-name "sensedSpeedReceived") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::setSpeedReceived"))) (name "setSpeedReceived") (declared-name "setSpeedReceived") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine::fuelCommandReceived"))) (name "fuelCommandReceived") (declared-name "fuelCommandReceived") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Event Occurrence Example::Engine")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (name "speedometer") (declared-name "speedometer") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer::sensedSpeedSent"))) (name "sensedSpeedSent") (declared-name "sensedSpeedSent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Event Occurrence Example::Speedometer")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Event Occurrence Example::driver"))) (to (node (document "d0") (qualified-name "Event Occurrence Example::Driver"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (to (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (to (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))) (to (node (document "d0") (qualified-name "Event Occurrence Example::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (to (node (document "d0") (qualified-name "Event Occurrence Example::Speedometer"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::CruiseController"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::Driver"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::Speedometer"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::driver"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::driver::setSpeedSent"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::fuelCommandSent"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::sensedSpeedReceived"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::cruiseController::setSpeedReceived"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::engine::fuelCommandReceived"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Event Occurrence Example::vehicle::speedometer::sensedSpeedSent"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/27_event_occurrence_example.md"
    (diagnostics
    )
  )
)
~~~
