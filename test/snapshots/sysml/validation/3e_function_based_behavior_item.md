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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "188dadad5702ecd46d6d9d7d59b3b8edfd9e1af1d6c0f6ca9166af5bae9117fc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item"))) (kind "package") (name "3e-Function-based Behavior-item") (declared-name "3e-Function-based Behavior-item") (range (start (line 0) (character 0)) (end (line 0) (character 1750))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 30))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 26))))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 3) (character 1)) (end (line 3) (character 198))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (kind "item def") (name "AssembledVehicle") (declared-name "AssembledVehicle") (range (start (line 6) (character 2)) (end (line 6) (character 47))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleAssembly") (range (start (line 6) (character 31)) (end (line 6) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 10) (character 2)) (end (line 10) (character 18))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 9) (character 2)) (end (line 9) (character 24))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 8) (character 2)) (end (line 8) (character 39))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AssembledVehicle") (range (start (line 8) (character 22)) (end (line 8) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))) (kind "item def") (name "VehicleAssembly") (declared-name "VehicleAssembly") (range (start (line 5) (character 2)) (end (line 5) (character 27))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 14) (character 1)) (end (line 14) (character 1470))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind "part") (name "AssemblyLine") (declared-name "AssemblyLine") (range (start (line 16) (character 2)) (end (line 16) (character 1446))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages"))) (authored (membership (kind Feature)) (relationships (perform (reference "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))) (kind "action") (name "assemble vehicle") (declared-name "assemble vehicle") (range (start (line 18) (character 3)) (end (line 18) (character 1100))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 50) (character 3)) (end (line 50) (character 218))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 50) (character 18)) (end (line 50) (character 25)))) (perform (reference "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower") (range none)))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 56) (character 4)) (end (line 56) (character 24))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 56) (character 17)) (end (line 56) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (range (start (line 58) (character 4)) (end (line 58) (character 32))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))))
    (element (id (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 55) (character 4)) (end (line 55) (character 36))) (parent (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 55) (character 23)) (end (line 55) (character 35)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 1) (character 15)) (end (line 1) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (kind specialization) (ordinal 0)) (authored-target "VehicleAssembly") (range (start (line 6) (character 31)) (end (line 6) (character 46))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind specialization) (ordinal 0)) (authored-target "AssembledVehicle") (range (start (line 8) (character 22)) (end (line 8) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind bindSource) (ordinal 0)) (authored-target "assemble vehicle::assemble engine into vehicle::assembledVehicle") (range (start (line 48) (character 8)) (end (line 48) (character 74))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind bindTarget) (ordinal 0)) (authored-target "vehicle") (range (start (line 48) (character 77)) (end (line 48) (character 84))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind performSource) (ordinal 0)) (authored-target "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 50) (character 18)) (end (line 50) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind performSource) (ordinal 0)) (authored-target "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 56) (character 17)) (end (line 56) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 55) (character 23)) (end (line 55) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
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
