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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 15) (end 1 29))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 16 2) (end 61 3))
      )
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
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1e15004c9dabb0f4a944ea895d010630217938a8b5e667538a4af55dacfbcadf") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehicleAssembly")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AssembledVehicle")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "vehicle")) (memberAccessOperand (reference "assemble vehicle::assemble engine into vehicle::assembledVehicle")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "assemble transmission into vehicle::vehicle assy without engine")) (flowTarget (reference "assemble engine into vehicle::vehicle assy without engine")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AssembledVehicle")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::engine"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleAssembly")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::transmission"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (documentation (comment (text " Note: A part can be treated as an item. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleAssembly")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine::transmission"))) (kind part) (membership (kind feature) (visibility default)) (documentation (comment (text " Note: An item can become a part of something else. "))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without transmission or engine"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleAssembly")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (documentation (comment (text "\n\t\t\t\t * Note: An in item one context can become a part in an other.\n\t\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (kind specialization) (ordinal 0))
      (authored-target "VehicleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind specialization) (ordinal 0))
      (authored-target "AssembledVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "assemble vehicle::assemble engine into vehicle::assembledVehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "assemble transmission into vehicle::vehicle assy without engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "assemble engine into vehicle::vehicle assy without engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "AssembledVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without transmission or engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle::engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine::transmission"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::transmission"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine::transmission"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without transmission or engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without transmission or engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle::engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine::transmission"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::transmission"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine::transmission"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without transmission or engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle::engine")) (scopes any))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::engine")) (scopes any))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine::transmission")) (scopes any))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::transmission")) (scopes any))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine::transmission")) (scopes any))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine")) (scopes any))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine")) (scopes any))
      (subtype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without transmission or engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (anonymous (kind bind) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine")))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine")))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle::engine")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::engine")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine::transmission")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::transmission")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine::transmission")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without transmission or engine")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::providePower")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission")))
      (featured-by (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle")))
      (type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (provenance authored))
      (effective-type (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (source direct))
      (supertype (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 1 15) (end 1 29)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 6 31) (end 6 46)) (probe (position 6 31))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle"))) (kind specialization) (ordinal 0) (authored-target "VehicleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 8 22) (end 8 38)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle"))) (kind specialization) (ordinal 0) (authored-target "AssembledVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 48 77) (end 48 84)) (probe (position 48 77))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 48 8) (end 48 74)) (probe (position 48 8))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "assemble vehicle::assemble engine into vehicle::assembledVehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 33 9) (end 33 75)) (probe (position 33 9))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "assemble transmission into vehicle::vehicle assy without engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 34 11) (end 34 71)) (probe (position 34 11))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (path (named (kind package) (name "3e-Function-based Behavior-item")) (named (kind package) (name "Usages")) (named (kind part) (name "AssemblyLine")) (named (kind perform-action) (name "assemble vehicle")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "assemble engine into vehicle::vehicle assy without engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 42 33) (end 42 49)) (probe (position 42 33))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "AssembledVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::AssembledVehicle")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 43 20) (end 43 26)) (probe (position 43 20))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::assembledVehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 40 22) (end 40 28)) (probe (position 40 22))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 37 45) (end 37 60)) (probe (position 37 45))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 38 26) (end 38 38)) (probe (position 38 26))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble engine into vehicle::vehicle assy without engine::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 22 28) (end 22 40)) (probe (position 22 28))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 26 46) (end 26 61)) (probe (position 26 46))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 27 26) (end 27 38)) (probe (position 27 26))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without engine::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 21 61) (end 21 76)) (probe (position 21 61))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::assemble vehicle::assemble transmission into vehicle::vehicle assy without transmission or engine"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::VehicleAssembly")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 50 18) (end 50 25)) (probe (position 50 18))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 56 17) (end 56 23)) (probe (position 56 17))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Engine")))))
    )
  )
  (query (document "memory://snapshot/3e_function_based_behavior_item.md") (range (start 55 23) (end 55 35)) (probe (position 55 23))
    (reference (id (source (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Usages::AssemblyLine::vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/3e_function_based_behavior_item.md") (qualified-name "3e-Function-based Behavior-item::Definitions::Transmission")))))
    )
  )
)
~~~
