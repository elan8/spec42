# META
~~~ini
description=SysML Training 31 (Constraints): Time Constraints
type=file
~~~
# SOURCE
~~~sysml
package 'Time Constraints' {
	private import ISQ::TemperatureValue;
	private import ISQ::DurationValue;
	private import Time::TimeInstantValue;
	private import Time::TimeOf;
	private import Time::DurationOf;
	private import SI::h;
	private import SI::s;

	attribute def MaintenanceDone;
	
	part def Vehicle {
		attribute maintenanceTime : TimeInstantValue;
		attribute maintenanceInterval : DurationValue;
		attribute maxTemperature : TemperatureValue;
	}
	
	state healthStates {
		in vehicle : Vehicle;
		
		entry; then normal;
		
		state normal;
		accept at vehicle.maintenanceTime
			then maintenance;
		
		state maintenance {
			assert constraint { TimeOf(maintenance) > vehicle.maintenanceTime }
			assert constraint { TimeOf(maintenance) - TimeOf(normal.done) < 2 [s] }
			entry assign vehicle.maintenanceTime := vehicle.maintenanceTime + vehicle.maintenanceInterval;
		}
		accept MaintenanceDone
			then normal;
		
		constraint { DurationOf(maintenance) <= 48 [h] }
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "31_time_constraints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 21))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Time Constraints' {
	private import ISQ::TemperatureValue;
	private import ISQ::DurationValue;
	private import Time::TimeInstantValue;
	private import Time::TimeOf;
	private import Time::DurationOf;
	private import SI::h;
	private import SI::s;

	attribute def MaintenanceDone;
	
	part def Vehicle {
		attribute maintenanceTime : TimeInstantValue;
		attribute maintenanceInterval : DurationValue;
		attribute maxTemperature : TemperatureValue;
	}
	
	state healthStates {
		in vehicle : Vehicle;
		
		entry; then normal;
		
		state normal;
		accept at vehicle.maintenanceTime
			then maintenance;
		
		state maintenance {
			assert constraint { TimeOf(maintenance) > vehicle.maintenanceTime }
			assert constraint { TimeOf(maintenance) - TimeOf(normal.done) < 2 [s] }
			entry assign vehicle.maintenanceTime := vehicle.maintenanceTime + vehicle.maintenanceInterval;
		}
		accept MaintenanceDone
			then normal;
		
		constraint { DurationOf(maintenance) <= 48 [h] }
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e19813f6ad4b925cc734672d4dbcc124d281d0de09fa06e6867988faa9e67378") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Time Constraints"))) (kind "package") (name "Time Constraints") (declared-name "Time Constraints"))
    (element (id (node (document "d0") (qualified-name "Time Constraints::DurationOf"))) (kind "import") (name "DurationOf") (declared-name "DurationOf") (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::DurationOf") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::DurationValue"))) (kind "import") (name "DurationValue") (declared-name "DurationValue") (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::DurationValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::MaintenanceDone"))) (kind "attribute def") (name "MaintenanceDone") (declared-name "MaintenanceDone") (parent (node (document "d0") (qualified-name "Time Constraints"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::TemperatureValue"))) (kind "import") (name "TemperatureValue") (declared-name "TemperatureValue") (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::TemperatureValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))) (kind "import") (name "TimeInstantValue") (declared-name "TimeInstantValue") (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::TimeInstantValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::TimeOf"))) (kind "import") (name "TimeOf") (declared-name "TimeOf") (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::TimeOf") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Time Constraints"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind "attribute") (name "maintenanceInterval") (declared-name "maintenanceInterval") (parent (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DurationValue")) (typing (reference "DurationValue")))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind "attribute") (name "maintenanceTime") (declared-name "maintenanceTime") (parent (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeInstantValue")) (typing (reference "TimeInstantValue")))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind "attribute") (name "maxTemperature") (declared-name "maxTemperature") (parent (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureValue")) (typing (reference "TemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::h"))) (kind "import") (name "h") (declared-name "h") (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::h") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind "state") (name "healthStates") (declared-name "healthStates") (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Feature)) (relationships (transition (reference "Time Constraints::healthStates::maintenance")) (transition (reference "Time Constraints::healthStates::normal")) (initial-state (reference "Time Constraints::healthStates::normal")))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::maintenance"))) (kind "state") (name "maintenance") (declared-name "maintenance") (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::normal"))) (kind "state") (name "normal") (declared-name "normal") (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_maintenance"))) (kind "transition") (name "transition_healthStates_to_maintenance") (declared-name "transition_healthStates_to_maintenance") (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_maintenance::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_maintenance"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_normal"))) (kind "transition") (name "transition_healthStates_to_normal") (declared-name "transition_healthStates_to_normal") (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Time Constraints::healthStates::transition_healthStates_to_normal"))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (kind "in out parameter") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Time Constraints::s"))) (kind "import") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "Time Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::s") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::DurationOf"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::DurationOf") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::DurationValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::DurationValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::TemperatureValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::TemperatureValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::TimeInstantValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::TimeOf"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::TimeOf") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 1)) (authored-target "DurationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "TimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::TemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::TemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::h"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::h") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind transitionSource) (ordinal 0)) (authored-target "Time Constraints::healthStates::maintenance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::maintenance")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind transitionSource) (ordinal 1)) (authored-target "Time Constraints::healthStates::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "Time Constraints::healthStates::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Constraints::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Time Constraints::s"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::s") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (target (node (document "d0") (qualified-name "Time Constraints::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (target (node (document "d0") (qualified-name "Time Constraints::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (target (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (target (node (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (target (node (document "d0") (qualified-name "Time Constraints::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (target (node (document "d0") (qualified-name "Time Constraints::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::maintenance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (target (node (document "d0") (qualified-name "Time Constraints::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::healthStates"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (target (node (document "d0") (qualified-name "Time Constraints::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Constraints::healthStates::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 16) (end 6 21)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::h"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::h")
        (range (start 6 16) (end 6 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 21)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::s"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::s")
        (range (start 7 16) (end 7 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 16) (end 4 28)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::TimeOf"))
        (kind membershipImport) (ordinal 0) (authored-target "Time::TimeOf")
        (range (start 4 16) (end 4 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 34) (end 13 47)) (probe (position 13 34))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))
        (kind featureTyping) (ordinal 1) (authored-target "DurationValue")
        (range (start 13 34) (end 13 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time Constraints::DurationValue") (range (start 2 1) (end 2 35)))
        )
      )
    )
    (query (range (start 5 16) (end 5 32)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::DurationOf"))
        (kind membershipImport) (ordinal 0) (authored-target "Time::DurationOf")
        (range (start 5 16) (end 5 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 30) (end 12 46)) (probe (position 12 30))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))
        (kind featureTyping) (ordinal 1) (authored-target "TimeInstantValue")
        (range (start 12 30) (end 12 46))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time Constraints::TimeInstantValue") (range (start 3 1) (end 3 39)))
        )
      )
    )
    (query (range (start 14 29) (end 14 45)) (probe (position 14 29))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::Vehicle::maxTemperature"))
        (kind featureTyping) (ordinal 1) (authored-target "TemperatureValue")
        (range (start 14 29) (end 14 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Time Constraints::TemperatureValue") (range (start 1 1) (end 1 38)))
        )
      )
    )
    (query (range (start 2 16) (end 2 34)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::DurationValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::DurationValue")
        (range (start 2 16) (end 2 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 37)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::TemperatureValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::TemperatureValue")
        (range (start 1 16) (end 1 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 38)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "Time Constraints::TimeInstantValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Time::TimeInstantValue")
        (range (start 3 16) (end 3 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
