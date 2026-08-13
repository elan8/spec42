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
  (document "memory://snapshot/25_change_and_time_triggers.md"
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
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 30) (end 9 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 34) (end 10 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 29) (end 11 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 16 27) (end 16 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 1) (end 41 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:cbefe7c7622310fd225d74f521948e918c56809e05ccc4e2afb4d222933bed44") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::TemperatureValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::DurationValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::TimeInstantValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::h") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::OverTemp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue"))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeInstantValue"))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue"))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::VehicleController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature"))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::h")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeInstantValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0))
      (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::TemperatureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 2 16) (end 2 34)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 3 16) (end 3 38)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 4 16) (end 4 21)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "SI::h")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 10 34) (end 10 47)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 9 30) (end 9 46)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0) (authored-target "TimeInstantValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 11 29) (end 11 45)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
)
~~~
