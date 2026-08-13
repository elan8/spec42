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
  (document "memory://snapshot/3e_function_based_behavior_item.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 2) (end 5 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 2) (end 6 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 8 22) (end 8 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 18 3) (end 46 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 48 3) (end 48 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 58 4) (end 58 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:1e15004c9dabb0f4a944ea895d010630217938a8b5e667538a4af55dacfbcadf") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AssembledVehicle"))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind specialization) (ordinal 0))
      (authored-target "AssembledVehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 1 15) (end 1 29)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions")))))
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 8 22) (end 8 38)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind specialization) (ordinal 0) (authored-target "AssembledVehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 50 18) (end 50 25)) (probe (position 50 18))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 56 17) (end 56 23)) (probe (position 56 17))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))))
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 55 23) (end 55 35)) (probe (position 55 23))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
  )
)
~~~
