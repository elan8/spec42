# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3d-Function-based Behavior-item
type=file
~~~
# SOURCE
~~~sysml
package '3d-Function-based Behavior-item' {
	private import ScalarValues::Real;
	public import Definitions::*;
	public import Usages::*;
	
	package Definitions {
		
		item def Fuel;
		
		port def FuelPort {
			out item fuel: Fuel;
		}
				
		part def Pump {
			port fuelInPort : ~FuelPort;
			port fuelOutPort : FuelPort;
		}
		
		part def StorageTank {
			port fuelOutPort : FuelPort;
		}
		
		part def FuelTank {
			port fuelInPort : ~FuelPort;
		}
		
		part def Vehicle {
			port fuelInPort : ~FuelPort;
		}
		
		action def PumpFuel {
			in fuelIn : Fuel;
			out fuelOut : Fuel;
		}
		
	}
	
	package Usages {
		
		part context {
			
			/* Storage Element */
			part storageTank : StorageTank;
			
			flow of  fuel : Fuel
				from storageTank.fuelOutPort.fuel to pump.fuelInPort.fuel {
				/*
				 * Note: Explicitly notating that the flow is "of fuel : Fuel" is optional.
				 */					
			}
			
			part pump : Pump {
				perform action pumpFuel : PumpFuel {
					in fuelIn = fuelInPort.fuel;
					out fuelOut = fuelOutPort.fuel;
				}
			}
			
			flow of fuel : Fuel
				from pump.fuelOutPort.fuel to vehicle.fuelInPort.fuel;
			
			part vehicle : Vehicle {
				flow fuelInPort.fuel to fuelTank.fuel {
					/* 
					 * Note: The semantics of flowing to a "stored item" is tentative.
					 */					
				}
				
				/* Storage Element */
				part fuelTank : FuelTank {
					attribute volumeMax : Real;
					attribute fuelLevel : Real = fuel.volume / volumeMax;
					
					 /* Stored Item */
					item fuel : Fuel {
						attribute volume : Real;
						/* isConserved = true */
					}
				}
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/3d_function_based_behavior_item.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 44 3) (end 49 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 52 4) (end 55 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 58 3) (end 59 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 62 4) (end 66 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 27) (end 70 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 27) (end 71 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 25) (end 75 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:a938ea0114bc41569429942e9fe5450deb09a9b292dd5b2393f311ceb44d8785") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Usages") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel") (direction in))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel") (direction out))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StorageTank"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTank"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuel"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuel::volume"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind featureTyping) (ordinal 0))
      (authored-target "StorageTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuel::volume"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuel"))) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 2 15) (end 2 29)) (probe (position 2 15))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 3 15) (end 3 24)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 10 18) (end 10 22)) (probe (position 10 18))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 23 22) (end 23 30)) (probe (position 23 22))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 14 22) (end 14 30)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 15 22) (end 15 30)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 31 15) (end 31 19)) (probe (position 31 15))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 32 17) (end 32 21)) (probe (position 32 17))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 19 22) (end 19 30)) (probe (position 19 22))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 27 22) (end 27 30)) (probe (position 27 22))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 51 15) (end 51 19)) (probe (position 51 15))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 42 22) (end 42 33)) (probe (position 42 22))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind featureTyping) (ordinal 0) (authored-target "StorageTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 61 18) (end 61 25)) (probe (position 61 18))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 69 20) (end 69 28)) (probe (position 69 20))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 74 17) (end 74 21)) (probe (position 74 17))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuel"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 75 25) (end 75 29)) (probe (position 75 25))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuel::volume"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 71 27) (end 71 31)) (probe (position 71 27))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3d_function_based_behavior_item.md") (range (start 70 27) (end 70 31)) (probe (position 70 27))
    (reference (id (source (node (document "memory://snapshot/3d_function_based_behavior_item.md") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
