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
  (document "3d_function_based_behavior_item.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 9) (end 45 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 41) (end 45 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 59 9) (end 59 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 59 34) (end 59 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 62 9) (end 62 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 62 28) (end 62 41))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "02150292b6fd5b57b9bea9ed1d25cbf9df2eeefe776d9665981897efb7c3f5a6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))) (kind "package") (name "3d-Function-based Behavior-item") (declared-name "3d-Function-based Behavior-item"))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))) (authored (membership (kind Import) (visibility "public") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (kind "item def") (name "Fuel") (declared-name "Fuel") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (kind "port def") (name "FuelPort") (declared-name "FuelPort") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind "item") (name "fuel") (declared-name "fuel") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::~FuelPort"))) (kind "conjugated port definition") (name "~FuelPort") (declared-name "~FuelPort") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (kind "part def") (name "FuelTank") (declared-name "FuelTank") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (kind "part def") (name "Pump") (declared-name "Pump") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind "port") (name "fuelOutPort") (declared-name "fuelOutPort") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPort")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (kind "action def") (name "PumpFuel") (declared-name "PumpFuel") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind "in out parameter") (name "fuelIn") (declared-name "fuelIn") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (authored (relationships (typing (reference "Fuel")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind "in out parameter") (name "fuelOut") (declared-name "fuelOut") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (authored (relationships (typing (reference "Fuel")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (kind "part def") (name "StorageTank") (declared-name "StorageTank") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind "port") (name "fuelOutPort") (declared-name "fuelOutPort") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPort")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind "part") (name "context") (declared-name "context") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages"))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind "part") (name "pump") (declared-name "pump") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Pump")) (perform (reference "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (kind "action") (name "pumpFuel") (declared-name "pumpFuel") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (authored (relationships (typing (reference "PumpFuel")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind "part") (name "storageTank") (declared-name "storageTank") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "StorageTank")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind "part") (name "fuelTank") (declared-name "fuelTank") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTank")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind "attribute") (name "fuelLevel") (declared-name "fuelLevel") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind "attribute") (name "volumeMax") (declared-name "volumeMax") (parent (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind flowSource) (ordinal 0)) (authored-target "storageTank::fuelOutPort::fuel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind flowSource) (ordinal 1)) (authored-target "pump::fuelOutPort::fuel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind flowTarget) (ordinal 0)) (authored-target "pump::fuelInPort::fuel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))) (kind flowTarget) (ordinal 1)) (authored-target "vehicle::fuelInPort::fuel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind featureTyping) (ordinal 0)) (authored-target "Pump") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind performSource) (ordinal 0)) (authored-target "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (kind featureTyping) (ordinal 0)) (authored-target "PumpFuel") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind featureTyping) (ordinal 0)) (authored-target "StorageTank") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "fuelInPort::fuel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind flowTarget) (ordinal 0)) (authored-target "fuelTank::fuel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTank") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort::fuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump::fuelOutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelIn"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel::fuelOut"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::PumpFuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump::pumpFuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (target (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))) (kind featureTyping) (ordinal 1)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 51 15) (end 51 19)) (probe (position 51 15))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::pump"))
        (kind featureTyping) (ordinal 0) (authored-target "Pump")
        (range (start 51 15) (end 51 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Pump") (range (start 13 2) (end 13 85)))
        )
      )
    )
    (query (range (start 70 27) (end 70 31)) (probe (position 70 27))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::volumeMax"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 70 27) (end 70 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3d-Function-based Behavior-item::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 71 27) (end 71 31)) (probe (position 71 27))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank::fuelLevel"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 71 27) (end 71 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3d-Function-based Behavior-item::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 3 15) (end 3 21)) (probe (position 3 15))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Usages::*")
        (range (start 3 15) (end 3 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages") (range (start 37 1) (end 37 1005)))
        )
      )
    )
    (query (range (start 61 18) (end 61 25)) (probe (position 61 18))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 61 18) (end 61 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::Vehicle") (range (start 26 2) (end 26 56)))
        )
      )
    )
    (query (range (start 69 20) (end 69 28)) (probe (position 69 20))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle::fuelTank"))
        (kind featureTyping) (ordinal 0) (authored-target "FuelTank")
        (range (start 69 20) (end 69 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::FuelTank") (range (start 22 2) (end 22 57)))
        )
      )
    )
    (query (range (start 2 15) (end 2 26)) (probe (position 2 15))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 2 15) (end 2 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions") (range (start 5 1) (end 5 452)))
        )
      )
    )
    (query (range (start 42 22) (end 42 33)) (probe (position 42 22))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::storageTank"))
        (kind featureTyping) (ordinal 0) (authored-target "StorageTank")
        (range (start 42 22) (end 42 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3d-Function-based Behavior-item::Definitions::StorageTank") (range (start 18 2) (end 18 60)))
        )
      )
    )
    (query (range (start 62 28) (end 62 41)) (probe (position 62 28))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))
        (kind flowTarget) (ordinal 0) (authored-target "fuelTank::fuel")
        (range (start 62 28) (end 62 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 62 9) (end 62 24)) (probe (position 62 9))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context::vehicle"))
        (kind flowSource) (ordinal 0) (authored-target "fuelInPort::fuel")
        (range (start 62 9) (end 62 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 41) (end 45 61)) (probe (position 45 41))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))
        (kind flowTarget) (ordinal 0) (authored-target "pump::fuelInPort::fuel")
        (range (start 45 41) (end 45 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 59 9) (end 59 30)) (probe (position 59 9))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))
        (kind flowSource) (ordinal 1) (authored-target "pump::fuelOutPort::fuel")
        (range (start 59 9) (end 59 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 59 34) (end 59 57)) (probe (position 59 34))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))
        (kind flowTarget) (ordinal 1) (authored-target "vehicle::fuelInPort::fuel")
        (range (start 59 34) (end 59 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 9) (end 45 37)) (probe (position 45 9))
      (reference
        (source (document "d0") (qualified-name "3d-Function-based Behavior-item::Usages::context"))
        (kind flowSource) (ordinal 0) (authored-target "storageTank::fuelOutPort::fuel")
        (range (start 45 9) (end 45 37))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
