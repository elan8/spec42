# META
~~~ini
description=SysML Training 25 (Transitions): Change and Time Triggers
type=file
~~~
# SOURCE
~~~sysml
package 'Change and Time Triggers' {
	private import ISQ::TemperatureValue;
	private import ISQ::DurationValue;
	private import Time::TimeInstantValue;
	private import SI::h;
	
	attribute def OverTemp;
	
	part def Vehicle {
		attribute maintenanceTime : TimeInstantValue;
		attribute maintenanceInterval : DurationValue;
		attribute maxTemperature : TemperatureValue;
	}
	
	part def VehicleController;
	
	action senseTemperature { out temp : TemperatureValue; }
	
	state healthStates {
		in vehicle : Vehicle;
		in controller : VehicleController;
		
		entry; then normal;
		do senseTemperature;
		
		state normal;
		accept at vehicle.maintenanceTime
			then maintenance;
		accept when senseTemperature.temp > vehicle.maxTemperature
			do send new OverTemp() to controller 
			then degraded;
		
		state maintenance {
			entry assign vehicle.maintenanceTime := vehicle.maintenanceTime + vehicle.maintenanceInterval;
		}
		accept after 48 [h]
			then normal;
		
		state degraded;
		accept when senseTemperature.temp <= vehicle.maxTemperature
			then normal;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "25_change_and_time_triggers.md"
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
        (range (start 4 16) (end 4 21))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Change and Time Triggers' {
    private import ISQ::TemperatureValue;
    private import ISQ::DurationValue;
    private import Time::TimeInstantValue;
    private import SI::h;

    attribute def OverTemp;

    part def Vehicle {
        attribute maintenanceTime : TimeInstantValue;
        attribute maintenanceInterval : DurationValue;
        attribute maxTemperature : TemperatureValue;
    }

    part def VehicleController;

    action senseTemperature { out temp : TemperatureValue; }

    state healthStates {
        in vehicle : Vehicle;
        in controller : VehicleController;

        entry; then normal;
        do senseTemperature;

        state normal;
        accept at vehicle.maintenanceTime
        then maintenance;
        accept when senseTemperature.temp > vehicle.maxTemperature
        do send new OverTemp() to controller
        then degraded;

        state maintenance {
            entry assign vehicle.maintenanceTime := vehicle.maintenanceTime + vehicle.maintenanceInterval;
        }
        accept after 48 [h]
        then normal;

        state degraded;
        accept when senseTemperature.temp <= vehicle.maxTemperature
        then normal;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a67f0f61e797c764451e07252a52fb1aa8070306601fa109967435e44fcbb786") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers"))) (kind "package") (name "Change and Time Triggers") (declared-name "Change and Time Triggers"))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))) (kind "import") (name "DurationValue") (declared-name "DurationValue") (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::DurationValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::OverTemp"))) (kind "attribute def") (name "OverTemp") (declared-name "OverTemp") (parent (node (document "d0") (qualified-name "Change and Time Triggers"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (kind "import") (name "TemperatureValue") (declared-name "TemperatureValue") (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::TemperatureValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))) (kind "import") (name "TimeInstantValue") (declared-name "TimeInstantValue") (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::TimeInstantValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Change and Time Triggers"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind "attribute") (name "maintenanceInterval") (declared-name "maintenanceInterval") (parent (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DurationValue")) (typing (reference "DurationValue")))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind "attribute") (name "maintenanceTime") (declared-name "maintenanceTime") (parent (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeInstantValue")) (typing (reference "TimeInstantValue")))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind "attribute") (name "maxTemperature") (declared-name "maxTemperature") (parent (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureValue")) (typing (reference "TemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::VehicleController"))) (kind "part def") (name "VehicleController") (declared-name "VehicleController") (parent (node (document "d0") (qualified-name "Change and Time Triggers"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::h"))) (kind "import") (name "h") (declared-name "h") (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::h") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind "state") (name "healthStates") (declared-name "healthStates") (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Feature)) (relationships (transition (reference "Change and Time Triggers::healthStates::maintenance")) (transition (reference "Change and Time Triggers::healthStates::degraded")) (transition (reference "Change and Time Triggers::healthStates::normal")) (transition (reference "Change and Time Triggers::healthStates::normal")) (initial-state (reference "Change and Time Triggers::healthStates::normal")))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::_do"))) (kind "action") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind "in out parameter") (name "controller") (declared-name "controller") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (authored (relationships (typing (reference "VehicleController")))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::degraded"))) (kind "state") (name "degraded") (declared-name "degraded") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::maintenance"))) (kind "state") (name "maintenance") (declared-name "maintenance") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))) (kind "state") (name "normal") (declared-name "normal") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded"))) (kind "transition") (name "transition_healthStates_to_degraded") (declared-name "transition_healthStates_to_degraded") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_maintenance"))) (kind "transition") (name "transition_healthStates_to_maintenance") (declared-name "transition_healthStates_to_maintenance") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_maintenance::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_maintenance"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal"))) (kind "transition") (name "transition_healthStates_to_normal") (declared-name "transition_healthStates_to_normal") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal#transition"))) (kind "transition") (name "transition_healthStates_to_normal") (declared-name "transition_healthStates_to_normal") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal#transition::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal#transition"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind "in out parameter") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature"))) (kind "action") (name "senseTemperature") (declared-name "senseTemperature") (parent (node (document "d0") (qualified-name "Change and Time Triggers"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (parent (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature"))) (authored (relationships (typing (reference "TemperatureValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::DurationValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::TemperatureValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::TimeInstantValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 1)) (authored-target "DurationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "TimeInstantValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::h"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::h") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 0)) (authored-target "Change and Time Triggers::healthStates::maintenance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::maintenance")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 1)) (authored-target "Change and Time Triggers::healthStates::degraded") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::degraded")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 2)) (authored-target "Change and Time Triggers::healthStates::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 3)) (authored-target "Change and Time Triggers::healthStates::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "Change and Time Triggers::healthStates::normal") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::VehicleController")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::degraded"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::maintenance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 3)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::VehicleController"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 16) (end 4 21)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "Change and Time Triggers::h"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::h")
        (range (start 4 16) (end 4 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 34) (end 10 47)) (probe (position 10 34))
      (reference
        (source (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))
        (kind featureTyping) (ordinal 1) (authored-target "DurationValue")
        (range (start 10 34) (end 10 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Change and Time Triggers::DurationValue") (range (start 2 1) (end 2 35)))
        )
      )
    )
    (query (range (start 9 30) (end 9 46)) (probe (position 9 30))
      (reference
        (source (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))
        (kind featureTyping) (ordinal 1) (authored-target "TimeInstantValue")
        (range (start 9 30) (end 9 46))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue") (range (start 3 1) (end 3 39)))
        )
      )
    )
    (query (range (start 11 29) (end 11 45)) (probe (position 11 29))
      (reference
        (source (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))
        (kind featureTyping) (ordinal 1) (authored-target "TemperatureValue")
        (range (start 11 29) (end 11 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue") (range (start 1 1) (end 1 38)))
        )
      )
    )
    (query (range (start 2 16) (end 2 34)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::DurationValue")
        (range (start 2 16) (end 2 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 37)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::TemperatureValue")
        (range (start 1 16) (end 1 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 38)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Time::TimeInstantValue")
        (range (start 3 16) (end 3 38))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
