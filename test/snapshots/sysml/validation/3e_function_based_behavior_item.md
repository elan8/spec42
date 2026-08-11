# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3e-Function-based Behavior-item
type=file
~~~
# SOURCE
~~~sysml
package '3e-Function-based Behavior-item' {
	public import Definitions::*;
	
	package Definitions {
		
		item def VehicleAssembly;
		item def AssembledVehicle :> VehicleAssembly;
		
		part def Vehicle :> AssembledVehicle;		
		part def Transmission;
		part def Engine;		
		
	}
	
	package Usages {
		
		part AssemblyLine {
		
			perform action 'assemble vehicle' {
				
				action 'assemble transmission into vehicle' {
					in item 'vehicle assy without transmission or engine' : VehicleAssembly;					
					in item transmission : Transmission {
						/* Note: A part can be treated as an item. */
					}
					
					out item 'vehicle assy without engine' : VehicleAssembly = 'vehicle assy without transmission or engine' {						
						part transmission : Transmission = 'assemble transmission into vehicle'.transmission {
							/* Note: An item can become a part of something else. */
						}
					}
				}
				
				flow 'assemble transmission into vehicle'.'vehicle assy without engine' 
				    to 'assemble engine into vehicle'.'vehicle assy without engine';
				
				action 'assemble engine into vehicle' {
					in item 'vehicle assy without engine' : VehicleAssembly {
						part transmission : Transmission;
					}
					in item engine : Engine;
					
					out item assembledVehicle : AssembledVehicle = 'vehicle assy without engine' {
						part engine : Engine = 'assemble engine into vehicle'.engine;
					}
				}
			}
			
			bind 'assemble vehicle'.'assemble engine into vehicle'.assembledVehicle = vehicle;
			
			part vehicle : Vehicle {
				/*
				 * Note: An in item one context can become a part in an other.
				 */
			
				part transmission: Transmission;
				part engine: Engine;
				
				perform action providePower;
			}
			
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3e_function_based_behavior_item.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 48 8) (end 48 74))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "188dadad5702ecd46d6d9d7d59b3b8edfd9e1af1d6c0f6ca9166af5bae9117fc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item"))) (kind "package") (name "3e-Function-based Behavior-item") (declared-name "3e-Function-based Behavior-item"))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (kind "item def") (name "AssembledVehicle") (declared-name "AssembledVehicle") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleAssembly")))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AssembledVehicle")))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))) (kind "item def") (name "VehicleAssembly") (declared-name "VehicleAssembly") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind "part") (name "AssemblyLine") (declared-name "AssemblyLine") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages"))) (authored (membership (kind Feature)) (relationships (perform (reference "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle")))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))) (kind "action") (name "assemble vehicle") (declared-name "assemble vehicle") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")) (perform (reference "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower")))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (kind specialization) (ordinal 0)) (authored-target "VehicleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind specialization) (ordinal 0)) (authored-target "AssembledVehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind bindSource) (ordinal 0)) (authored-target "assemble vehicle::assemble engine into vehicle::assembledVehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind bindTarget) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind performSource) (ordinal 0)) (authored-target "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind performSource) (ordinal 0)) (authored-target "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower") (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 56 17) (end 56 23)) (probe (position 56 17))
      (reference
        (source (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 56 17) (end 56 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine") (range (start 10 2) (end 10 18)))
        )
      )
    )
    (query (range (start 48 77) (end 48 84)) (probe (position 48 77))
      (reference
        (source (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))
        (kind bindTarget) (ordinal 0) (authored-target "vehicle")
        (range (start 48 77) (end 48 84))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle") (range (start 50 3) (end 50 218)))
        )
      )
    )
    (query (range (start 50 18) (end 50 25)) (probe (position 50 18))
      (reference
        (source (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 50 18) (end 50 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle") (range (start 8 2) (end 8 39)))
        )
      )
    )
    (query (range (start 1 15) (end 1 26)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "3e-Function-based Behavior-item::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 1 15) (end 1 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions") (range (start 3 1) (end 3 198)))
        )
      )
    )
    (query (range (start 55 23) (end 55 35)) (probe (position 55 23))
      (reference
        (source (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 55 23) (end 55 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission") (range (start 9 2) (end 9 24)))
        )
      )
    )
    (query (range (start 6 31) (end 6 46)) (probe (position 6 31))
      (reference
        (source (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))
        (kind specialization) (ordinal 0) (authored-target "VehicleAssembly")
        (range (start 6 31) (end 6 46))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly") (range (start 5 2) (end 5 27)))
        )
      )
    )
    (query (range (start 8 22) (end 8 38)) (probe (position 8 22))
      (reference
        (source (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))
        (kind specialization) (ordinal 0) (authored-target "AssembledVehicle")
        (range (start 8 22) (end 8 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle") (range (start 6 2) (end 6 47)))
        )
      )
    )
    (query (range (start 48 8) (end 48 74)) (probe (position 48 8))
      (reference
        (source (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))
        (kind bindSource) (ordinal 0) (authored-target "assemble vehicle::assemble engine into vehicle::assembledVehicle")
        (range (start 48 8) (end 48 74))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
