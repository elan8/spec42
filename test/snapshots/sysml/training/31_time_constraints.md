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
  (document "memory://snapshot/31_time_constraints.md"
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
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 30) (end 12 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 34) (end 13 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 29) (end 14 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 17 1) (end 35 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:0162c5a148ec814d505bd1d3f39a960a673b833c10a6374b970ca83ff1c66f1d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::TemperatureValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::DurationValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::TimeInstantValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::TimeOf") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::DurationOf") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::h") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::s") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::MaintenanceDone"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue"))))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeInstantValue"))))
    (declaration (id (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::TimeOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::DurationOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::h")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::s")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeInstantValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0))
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
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::TemperatureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 2 16) (end 2 34)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 3 16) (end 3 38)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 4 16) (end 4 28)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Time::TimeOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 5 16) (end 5 32)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Time::DurationOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 6 16) (end 6 21)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "SI::h")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 7 16) (end 7 21)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "SI::s")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 13 34) (end 13 47)) (probe (position 13 34))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 12 30) (end 12 46)) (probe (position 12 30))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0) (authored-target "TimeInstantValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_time_constraints.md") (range (start 14 29) (end 14 45)) (probe (position 14 29))
    (reference (id (source (node (document "memory://snapshot/31_time_constraints.md") (qualified-name "Time Constraints::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
)
~~~
