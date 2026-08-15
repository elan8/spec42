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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 38) (end 16 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 28 2) (end 30 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 14) (end 28 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 14) (end 39 35))
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
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::TemperatureValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::DurationValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::TimeInstantValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::h") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::OverTemp"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue")))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeInstantValue")))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue")))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::VehicleController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "normal")))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "senseTemperature")))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "maintenance")) (memberAccessOperand (reference "vehicle::maintenanceTime")))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "degraded")) (memberAccessOperand (reference "senseTemperature::temp")) (memberAccessOperand (reference "vehicle::maxTemperature")))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 2))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "normal")))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "normal")) (memberAccessOperand (reference "senseTemperature::temp")) (memberAccessOperand (reference "vehicle::maxTemperature")))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleController") (direction in)))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::degraded"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::maintenance"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle") (direction in)))))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TemperatureValue") (direction out)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::TemperatureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
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
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "senseTemperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "maintenance")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::maintenance")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "degraded")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::degraded")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0))
      (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (kind transitionTarget) (ordinal 0))
      (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::maintenanceTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "senseTemperature::temp")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "senseTemperature::temp")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "vehicle::maxTemperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "vehicle::maxTemperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::VehicleController")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (kind featureTyping) (ordinal 0))
      (authored-target "TemperatureValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind doActionBinding) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::maintenance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::degraded"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::controller"))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::VehicleController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle")))
      (subtype (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval")))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime")))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature")))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::VehicleController")))
      (subtype (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::controller")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind do-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::controller")))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
      (type (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::VehicleController")) (provenance authored))
      (effective-type (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::VehicleController")) (source direct))
      (supertype (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::VehicleController")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::degraded")))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::maintenance")))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal")))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::vehicle")))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates")))
      (type (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature::temp")))
      (featured-by (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::TemperatureValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 2 16) (end 2 34)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 3 16) (end 3 38)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 4 16) (end 4 21)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "SI::h")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 10 34) (end 10 47)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 9 30) (end 9 46)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0) (authored-target "TimeInstantValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 11 29) (end 11 45)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 23 5) (end 23 21)) (probe (position 23 5))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "senseTemperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 22 14) (end 22 20)) (probe (position 22 14))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 27 8) (end 27 19)) (probe (position 27 8))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "maintenance")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::maintenance")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 30 8) (end 30 16)) (probe (position 30 8))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "degraded")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::degraded")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 36 8) (end 36 14)) (probe (position 36 8))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0) (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 40 8) (end 40 14)) (probe (position 40 8))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (kind transitionTarget) (ordinal 0) (authored-target "normal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 26 12) (end 26 35)) (probe (position 26 12))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::maintenanceTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 28 14) (end 28 35)) (probe (position 28 14))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "senseTemperature::temp")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 39 14) (end 39 35)) (probe (position 39 14))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0) (authored-target "senseTemperature::temp")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 28 38) (end 28 60)) (probe (position 28 38))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1) (authored-target "vehicle::maxTemperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 39 39) (end 39 61)) (probe (position 39 39))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (path (named (kind package) (name "Change and Time Triggers")) (named (kind state) (name "healthStates")) (anonymous (kind transition) (ordinal 3))))) (kind memberAccessOperand) (ordinal 1) (authored-target "vehicle::maxTemperature")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 20 18) (end 20 35)) (probe (position 20 18))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::VehicleController")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 19 15) (end 19 22)) (probe (position 19 15))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/25_change_and_time_triggers.md") (range (start 16 38) (end 16 54)) (probe (position 16 38))
    (reference (id (source (node (document "memory://snapshot/25_change_and_time_triggers.md") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (kind featureTyping) (ordinal 0) (authored-target "TemperatureValue")
      (outcome (status unresolved)))
    )
  )
)
~~~
