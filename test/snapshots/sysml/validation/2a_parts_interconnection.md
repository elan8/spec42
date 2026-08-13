# META
~~~ini
description=SysML Validation (02-Parts Interconnection): 2a-Parts Interconnection
type=file
~~~
# SOURCE
~~~sysml
package '2a-Parts Interconnection' {
	public import Definitions::*;
	public import Usages::*;

	package Definitions {		
		// Port Definitions
		
		port def FuelCmdPort;
		
		port def DrivePwrPort;
		port def ClutchPort;
		
		port def ShaftPort_a;
		port def ShaftPort_b;
		port def ShaftPort_c;
		port def ShaftPort_d;
		
		port def DiffPort;
		port def AxlePort;
		port def AxleToWheelPort;
		port def WheelToAxlePort;
		port def WheelToRoadPort;
		
		port def VehicleToRoadPort {
			/*
			 * A port definition can have nested ports.
			 */
		 
			port wheelToRoadPort: WheelToRoadPort[2];
		}
	
		// Blocks
	
		part def VehicleA { 
			port fuelCmdPort: FuelCmdPort;
			port vehicleToRoadPort: VehicleToRoadPort;
		}
		
		part def AxleAssembly;		
		part def RearAxleAssembly :> AxleAssembly { 
			port shaftPort_d: ShaftPort_d;
		}
		
		part def Axle;
		part def RearAxle :> Axle;
		
		part def HalfAxle { 
			port axleToDiffPort: AxlePort;
			port axleToWheelPort: AxleToWheelPort;
		}
		
		part def Engine { 
			port fuelCmdPort: FuelCmdPort;
			port drivePwrPort: DrivePwrPort;
		}
	
		part def Transmission { 
			port clutchPort: ClutchPort;
			port shaftPort_a: ShaftPort_a;
		}
		
		part def Driveshaft { 
			port shaftPort_b: ShaftPort_b;
			port shaftPort_c: ShaftPort_c;
		}	
		
		part def Differential {
			/*
			 * Ports do not have to be defined on part defs.
			 * They can be added directly to their usages.
			 */
		}
		part def Wheel;
		
		// Interface Definitions
		
		interface def EngineToTransmissionInterface {
			/*
			 * The ends of an interface definition are always ports.
			 */
		
			end drivePwrPort: DrivePwrPort;
			end clutchPort: ClutchPort;
		}
		
		interface def DriveshaftInterface {
			end shaftPort_a: ShaftPort_a;
			end shaftPort_d: ShaftPort_d;
			
			ref driveshaft: Driveshaft {
				/*
				 * 'driveshaft' is a reference to the driveshaft that will
				 * act as the "interface medium" for this interface.
				 */
			}
			
			connect shaftPort_a to driveshaft.shaftPort_b {
				/*
				 * The two ends of 'DriveShaftInterface' are always connected
				 * via the referenced 'driveshaft'.
				 */
			}
			connect driveshaft.shaftPort_c to shaftPort_d;
		}
		
	}
	
	package Usages {
	
		part vehicle1_c1: VehicleA {
						
			bind fuelCmdPort = engine.fuelCmdPort;
			
			part engine: Engine;
			
			interface :EngineToTransmissionInterface
				connect engine.drivePwrPort to transmission.clutchPort {
				/*
				 * A usage of an interface definition connects two ports relative to 
				 * a containing context.
				 */
			}
				
			part transmission: Transmission;
			
			part driveshaft: Driveshaft {
				/*
				 * This 'driveshaft' is the part of 'vehicle1_c1' that will act as the
				 * interface medium in the following 'DriveshaftInterface' usage.
				 */
			}
			
			interface :DriveshaftInterface
				connect transmission.shaftPort_a to rearAxleAssembly.shaftPort_d {
					ref :>> driveshaft = vehicle1_c1.driveshaft {
						/*
						 * The reference property from 'DriveshaftInterface' is redefined
						 * in order to bind it to the appropriate part of 'vehicle1_c1'.
						 */
					}
				}
	
			part rearAxleAssembly: RearAxleAssembly {
				bind shaftPort_d = differential.shaftPort_d;
				
				part differential: Differential {
					port shaftPort_d: ShaftPort_d {
						/*
						 * If the part def has no ports, then they can be defined directly in
						 * a usage of the part def.
						 */
					}
					port leftDiffPort: DiffPort;
					port rightDiffPort: DiffPort;
				}
				
				interface differential.leftDiffPort to rearAxle.leftHalfAxle.axleToDiffPort {
					/*
					 * A connection can be to a port that is arbitrarily deeply nested, on either end. 
					 */
				}
				interface differential.rightDiffPort to rearAxle.rightHalfAxle.axleToDiffPort;
		
				part rearAxle: RearAxle {
					part leftHalfAxle: HalfAxle;
					part rightHalfAxle: HalfAxle;
				}
				
				connect rearAxle.leftHalfAxle.axleToWheelPort to leftWheel.wheelToAxlePort;
				connect rearAxle.rightHalfAxle.axleToWheelPort to rightWheel.wheelToAxlePort;
	
				part rearWheel: Wheel[2] ordered;
				
				/* The two rear wheels of 'rearAxleAssembly' must be given
				 * their own names in order to be referenced in connections.
				 * 
				 * (":>" is a shorthand here for "subsets".)
				 */
				part leftWheel :> rearWheel = rearWheel#(1) {
					port wheelToAxlePort: WheelToAxlePort;
					port wheelToRoadPort: WheelToRoadPort;
				}
				
				part rightWheel :> rearWheel = rearWheel#(2) {
					port wheelToAxlePort: WheelToAxlePort;
					port wheelToRoadPort: WheelToRoadPort;
				}
				
			}
			
			bind rearAxleAssembly.leftWheel.wheelToRoadPort = 
				 vehicleToRoadPort.leftWheelToRoadPort;
				 
			bind rearAxleAssembly.rightWheel.wheelToRoadPort = 
				 vehicleToRoadPort.rightWheelToRoadPort;
				
			port vehicleToRoadPort redefines VehicleA::vehicleToRoadPort {
				port leftWheelToRoadPort :> wheelToRoadPort = wheelToRoadPort#(1);
				port rightWheelToRoadPort :> wheelToRoadPort = wheelToRoadPort#(2);
			}
			
		}
	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/2a_parts_interconnection.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 76 2) (end 83 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 85 2) (end 103 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 111 3) (end 111 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 115 3) (end 121 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 132 3) (end 140 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 143 4) (end 143 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 156 4) (end 160 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 161 4) (end 161 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 168 4) (end 168 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 169 4) (end 169 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 178 22) (end 178 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 183 23) (end 183 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 190 3) (end 191 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 193 3) (end 194 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 196 36) (end 196 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 197 32) (end 197 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 198 33) (end 198 48))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:a8ad92c9854699c8eba68e11d0450fe3c79664cbeca150110178469c7cddc946") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Usages") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Axle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Differential"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_b"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_c"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePwrPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmdPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxlePort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleToWheelPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_d"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClutchPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_a"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmdPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleA"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driveshaft"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Differential"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DiffPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DiffPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_d"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "rearWheel"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToAxlePort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RearAxle"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HalfAxle"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HalfAxle"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "rearWheel"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToAxlePort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "VehicleA::vehicleToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "wheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "wheelToRoadPort"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_c")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleToWheelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (kind specialization) (ordinal 0))
      (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Axle")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (kind specialization) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driveshaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "RearAxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (kind featureTyping) (ordinal 0))
      (authored-target "Differential")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Differential")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiffPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiffPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (kind subsetting) (ordinal 0))
      (authored-target "rearWheel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToAxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "RearAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "HalfAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "HalfAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (kind subsetting) (ordinal 0))
      (authored-target "rearWheel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToAxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (kind redefinition) (ordinal 0))
      (authored-target "VehicleA::vehicleToRoadPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (kind subsetting) (ordinal 0))
      (authored-target "wheelToRoadPort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (kind subsetting) (ordinal 0))
      (authored-target "wheelToRoadPort")
      (outcome (status unsupported)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Differential"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 1 15) (end 1 29)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 2 15) (end 2 24)) (probe (position 2 15))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 62 21) (end 62 32)) (probe (position 62 21))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 63 21) (end 63 32)) (probe (position 63 21))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_c")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 53 22) (end 53 34)) (probe (position 53 22))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 52 21) (end 52 32)) (probe (position 52 21))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 47 24) (end 47 32)) (probe (position 47 24))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (kind featureTyping) (ordinal 0) (authored-target "AxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 48 25) (end 48 40)) (probe (position 48 25))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (kind featureTyping) (ordinal 0) (authored-target "AxleToWheelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 44 23) (end 44 27)) (probe (position 44 23))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (kind specialization) (ordinal 0) (authored-target "Axle")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Axle")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 39 31) (end 39 43)) (probe (position 39 31))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (kind specialization) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 40 21) (end 40 32)) (probe (position 40 21))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 57 20) (end 57 30)) (probe (position 57 20))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0) (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 58 21) (end 58 32)) (probe (position 58 21))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_a")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 34 21) (end 34 32)) (probe (position 34 21))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 35 27) (end 35 44)) (probe (position 35 27))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 28 25) (end 28 40)) (probe (position 28 25))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 109 20) (end 109 28)) (probe (position 109 20))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleA")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 125 20) (end 125 30)) (probe (position 125 20))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (kind featureTyping) (ordinal 0) (authored-target "Driveshaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 113 16) (end 113 22)) (probe (position 113 16))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Engine")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 142 26) (end 142 42)) (probe (position 142 26))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "RearAxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 145 23) (end 145 35)) (probe (position 145 23))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (kind featureTyping) (ordinal 0) (authored-target "Differential")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Differential")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 152 24) (end 152 32)) (probe (position 152 24))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (kind featureTyping) (ordinal 0) (authored-target "DiffPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 153 25) (end 153 33)) (probe (position 153 25))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (kind featureTyping) (ordinal 0) (authored-target "DiffPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 146 23) (end 146 34)) (probe (position 146 23))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 178 22) (end 178 31)) (probe (position 178 22))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (kind subsetting) (ordinal 0) (authored-target "rearWheel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 179 27) (end 179 42)) (probe (position 179 27))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToAxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 180 27) (end 180 42)) (probe (position 180 27))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 163 19) (end 163 27)) (probe (position 163 19))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0) (authored-target "RearAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 164 24) (end 164 32)) (probe (position 164 24))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0) (authored-target "HalfAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 165 25) (end 165 33)) (probe (position 165 25))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0) (authored-target "HalfAxle")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 171 20) (end 171 25)) (probe (position 171 20))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Wheel")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 183 23) (end 183 32)) (probe (position 183 23))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (kind subsetting) (ordinal 0) (authored-target "rearWheel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 184 27) (end 184 42)) (probe (position 184 27))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToAxlePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 185 27) (end 185 42)) (probe (position 185 27))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 123 22) (end 123 34)) (probe (position 123 22))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Definitions::Transmission")))))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 196 36) (end 196 63)) (probe (position 196 36))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (kind redefinition) (ordinal 0) (authored-target "VehicleA::vehicleToRoadPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 197 32) (end 197 47)) (probe (position 197 32))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (kind subsetting) (ordinal 0) (authored-target "wheelToRoadPort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/2a_parts_interconnection.md") (range (start 198 33) (end 198 48)) (probe (position 198 33))
    (reference (id (source (node (document "memory://snapshot/2a_parts_interconnection.md") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (kind subsetting) (ordinal 0) (authored-target "wheelToRoadPort")
      (outcome (status unsupported)))
  )
)
~~~
