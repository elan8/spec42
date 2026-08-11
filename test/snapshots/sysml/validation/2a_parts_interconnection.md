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
  (document "2a_parts_interconnection.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 96 26) (end 96 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 102 11) (end 102 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 111 8) (end 111 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 111 22) (end 111 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 116 12) (end 116 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 116 35) (end 116 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 133 12) (end 133 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 133 40) (end 133 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 143 9) (end 143 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 156 43) (end 156 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 161 44) (end 161 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 168 12) (end 168 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 169 12) (end 169 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 196 36) (end 196 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 197 32) (end 197 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 198 33) (end 198 48))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f7603000b796ef6ea75526c50e4466c5ff658b0ad75325de723d69a52c7e5585") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection"))) (kind "package") (name "2a-Parts Interconnection") (declared-name "2a-Parts Interconnection"))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection"))) (authored (membership (kind Import) (visibility "public") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Axle"))) (kind "part def") (name "Axle") (declared-name "Axle") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly"))) (kind "part def") (name "AxleAssembly") (declared-name "AxleAssembly") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))) (kind "port def") (name "AxlePort") (declared-name "AxlePort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort::~AxlePort"))) (kind "conjugated port definition") (name "~AxlePort") (declared-name "~AxlePort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))) (kind "port def") (name "AxleToWheelPort") (declared-name "AxleToWheelPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort::~AxleToWheelPort"))) (kind "conjugated port definition") (name "~AxleToWheelPort") (declared-name "~AxleToWheelPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))) (kind "port def") (name "ClutchPort") (declared-name "ClutchPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort::~ClutchPort"))) (kind "conjugated port definition") (name "~ClutchPort") (declared-name "~ClutchPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))) (kind "port def") (name "DiffPort") (declared-name "DiffPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort::~DiffPort"))) (kind "conjugated port definition") (name "~DiffPort") (declared-name "~DiffPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential"))) (kind "part def") (name "Differential") (declared-name "Differential") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))) (kind "port def") (name "DrivePwrPort") (declared-name "DrivePwrPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort::~DrivePwrPort"))) (kind "conjugated port definition") (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (kind "part def") (name "Driveshaft") (declared-name "Driveshaft") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (kind "port") (name "shaftPort_b") (declared-name "shaftPort_b") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_b")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (kind "port") (name "shaftPort_c") (declared-name "shaftPort_c") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_c")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind "interface def") (name "DriveshaftInterface") (declared-name "DriveshaftInterface") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft"))) (kind "ref") (name "driveshaft") (declared-name "driveshaft") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driveshaft")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a"))) (kind "interface end") (name "shaftPort_a") (declared-name "shaftPort_a") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (authored (relationships (typing (reference "ShaftPort_a")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d"))) (kind "interface end") (name "shaftPort_d") (declared-name "shaftPort_d") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (authored (relationships (typing (reference "ShaftPort_d")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "DrivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelCmdPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface"))) (kind "interface def") (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::clutchPort"))) (kind "interface end") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "ClutchPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind "interface end") (name "drivePwrPort") (declared-name "drivePwrPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "DrivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))) (kind "port def") (name "FuelCmdPort") (declared-name "FuelCmdPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort::~FuelCmdPort"))) (kind "conjugated port definition") (name "~FuelCmdPort") (declared-name "~FuelCmdPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (kind "part def") (name "HalfAxle") (declared-name "HalfAxle") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (kind "port") (name "axleToDiffPort") (declared-name "axleToDiffPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxlePort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (kind "port") (name "axleToWheelPort") (declared-name "axleToWheelPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleToWheelPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (kind "part def") (name "RearAxle") (declared-name "RearAxle") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Axle")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (kind "part def") (name "RearAxleAssembly") (declared-name "RearAxleAssembly") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (kind "port") (name "shaftPort_d") (declared-name "shaftPort_d") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_d")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))) (kind "port def") (name "ShaftPort_a") (declared-name "ShaftPort_a") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a::~ShaftPort_a"))) (kind "conjugated port definition") (name "~ShaftPort_a") (declared-name "~ShaftPort_a") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))) (kind "port def") (name "ShaftPort_b") (declared-name "ShaftPort_b") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b::~ShaftPort_b"))) (kind "conjugated port definition") (name "~ShaftPort_b") (declared-name "~ShaftPort_b") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))) (kind "port def") (name "ShaftPort_c") (declared-name "ShaftPort_c") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c::~ShaftPort_c"))) (kind "conjugated port definition") (name "~ShaftPort_c") (declared-name "~ShaftPort_c") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))) (kind "port def") (name "ShaftPort_d") (declared-name "ShaftPort_d") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d::~ShaftPort_d"))) (kind "conjugated port definition") (name "~ShaftPort_d") (declared-name "~ShaftPort_d") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClutchPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (kind "port") (name "shaftPort_a") (declared-name "shaftPort_a") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_a")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (kind "part def") (name "VehicleA") (declared-name "VehicleA") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelCmdPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (kind "port") (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleToRoadPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))) (kind "port def") (name "VehicleToRoadPort") (declared-name "VehicleToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::~VehicleToRoadPort"))) (kind "conjugated port definition") (name "~VehicleToRoadPort") (declared-name "~VehicleToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))) (kind "port def") (name "WheelToAxlePort") (declared-name "WheelToAxlePort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort::~WheelToAxlePort"))) (kind "conjugated port definition") (name "~WheelToAxlePort") (declared-name "~WheelToAxlePort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (kind "port def") (name "WheelToRoadPort") (declared-name "WheelToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort::~WheelToRoadPort"))) (kind "conjugated port definition") (name "~WheelToRoadPort") (declared-name "~WheelToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (kind "part") (name "driveshaft") (declared-name "driveshaft") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driveshaft")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "RearAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (kind "part") (name "differential") (declared-name "differential") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Differential")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (kind "port") (name "leftDiffPort") (declared-name "leftDiffPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (authored (membership (kind Feature)) (relationships (typing (reference "DiffPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (kind "port") (name "rightDiffPort") (declared-name "rightDiffPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (authored (membership (kind Feature)) (relationships (typing (reference "DiffPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (kind "port") (name "shaftPort_d") (declared-name "shaftPort_d") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_d")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (kind "part") (name "leftWheel") (declared-name "leftWheel") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (kind "port") (name "wheelToAxlePort") (declared-name "wheelToAxlePort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToAxlePort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "RearAxle")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind "part") (name "leftHalfAxle") (declared-name "leftHalfAxle") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "HalfAxle")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind "part") (name "rightHalfAxle") (declared-name "rightHalfAxle") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "HalfAxle")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (kind "part") (name "rightWheel") (declared-name "rightWheel") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind "port") (name "wheelToAxlePort") (declared-name "wheelToAxlePort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToAxlePort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (kind "port") (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleA::vehicleToRoadPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (kind "port") (name "leftWheelToRoadPort") (declared-name "leftWheelToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "wheelToRoadPort")))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (kind "port") (name "rightWheelToRoadPort") (declared-name "rightWheelToRoadPort") (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "wheelToRoadPort")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_b") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_c") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind connectionSource) (ordinal 0)) (authored-target "shaftPort_a") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind connectionSource) (ordinal 1)) (authored-target "driveshaft::shaftPort_c") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind connectionTarget) (ordinal 0)) (authored-target "driveshaft::shaftPort_b") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind connectionTarget) (ordinal 1)) (authored-target "shaftPort_d") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft"))) (kind featureTyping) (ordinal 0)) (authored-target "Driveshaft") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_a") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_d") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmdPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (kind featureTyping) (ordinal 0)) (authored-target "AxlePort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleToWheelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (kind specialization) (ordinal 0)) (authored-target "Axle") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Axle")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (kind specialization) (ordinal 0)) (authored-target "AxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_d") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_a") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmdPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind connectionSource) (ordinal 1)) (authored-target "engine::drivePwrPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind connectionSource) (ordinal 2)) (authored-target "transmission::shaftPort_a") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind connectionTarget) (ordinal 1)) (authored-target "transmission::clutchPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind connectionTarget) (ordinal 2)) (authored-target "rearAxleAssembly::shaftPort_d") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 0)) (authored-target "fuelCmdPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 3)) (authored-target "rearAxleAssembly::leftWheel::wheelToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 4)) (authored-target "rearAxleAssembly::rightWheel::wheelToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindTarget) (ordinal 0)) (authored-target "engine::fuelCmdPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindTarget) (ordinal 3)) (authored-target "vehicleToRoadPort::leftWheelToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindTarget) (ordinal 4)) (authored-target "vehicleToRoadPort::rightWheelToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (kind featureTyping) (ordinal 0)) (authored-target "Driveshaft") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "RearAxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionSource) (ordinal 1)) (authored-target "differential::leftDiffPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionSource) (ordinal 2)) (authored-target "differential::rightDiffPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionSource) (ordinal 3)) (authored-target "rearAxle::leftHalfAxle::axleToWheelPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionSource) (ordinal 4)) (authored-target "rearAxle::rightHalfAxle::axleToWheelPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionTarget) (ordinal 1)) (authored-target "rearAxle::leftHalfAxle::axleToDiffPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionTarget) (ordinal 2)) (authored-target "rearAxle::rightHalfAxle::axleToDiffPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionTarget) (ordinal 3)) (authored-target "leftWheel::wheelToAxlePort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionTarget) (ordinal 4)) (authored-target "rightWheel::wheelToAxlePort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind bindSource) (ordinal 0)) (authored-target "shaftPort_d") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind bindTarget) (ordinal 0)) (authored-target "differential::shaftPort_d") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (kind featureTyping) (ordinal 0)) (authored-target "Differential") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DiffPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DiffPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_d") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToAxlePort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "RearAxle") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "HalfAxle") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "HalfAxle") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToAxlePort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleA::vehicleToRoadPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (kind subsetting) (ordinal 0)) (authored-target "wheelToRoadPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (kind subsetting) (ordinal 0)) (authored-target "wheelToRoadPort") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::clutchPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::drivePwrPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Axle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 3)) (expression (kind bind) (source "rearAxleAssembly::leftWheel::wheelToRoadPort") (target "vehicleToRoadPort::leftWheelToRoadPort")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 4)) (expression (kind bind) (source "rearAxleAssembly::rightWheel::wheelToRoadPort") (target "vehicleToRoadPort::rightWheelToRoadPort")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 44 23) (end 44 27)) (probe (position 44 23))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))
        (kind specialization) (ordinal 0) (authored-target "Axle")
        (range (start 44 23) (end 44 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Axle") (range (start 43 2) (end 43 16)))
        )
      )
    )
    (query (range (start 171 20) (end 171 25)) (probe (position 171 20))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 171 20) (end 171 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Wheel") (range (start 72 2) (end 72 17)))
        )
      )
    )
    (query (range (start 2 15) (end 2 21)) (probe (position 2 15))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Usages::*")
        (range (start 2 15) (end 2 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages") (range (start 107 1) (end 107 2863)))
        )
      )
    )
    (query (range (start 113 16) (end 113 22)) (probe (position 113 16))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 113 16) (end 113 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine") (range (start 51 2) (end 51 94)))
        )
      )
    )
    (query (range (start 109 20) (end 109 28)) (probe (position 109 20))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleA")
        (range (start 109 20) (end 109 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA") (range (start 33 2) (end 33 106)))
        )
      )
    )
    (query (range (start 163 19) (end 163 27)) (probe (position 163 19))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "RearAxle")
        (range (start 163 19) (end 163 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle") (range (start 44 2) (end 44 28)))
        )
      )
    )
    (query (range (start 164 24) (end 164 32)) (probe (position 164 24))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "HalfAxle")
        (range (start 164 24) (end 164 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle") (range (start 46 2) (end 46 102)))
        )
      )
    )
    (query (range (start 165 25) (end 165 33)) (probe (position 165 25))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "HalfAxle")
        (range (start 165 25) (end 165 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle") (range (start 46 2) (end 46 102)))
        )
      )
    )
    (query (range (start 178 22) (end 178 31)) (probe (position 178 22))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))
        (kind subsetting) (ordinal 0) (authored-target "rearWheel")
        (range (start 178 22) (end 178 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel") (range (start 171 4) (end 171 37)))
        )
      )
    )
    (query (range (start 183 23) (end 183 32)) (probe (position 183 23))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))
        (kind subsetting) (ordinal 0) (authored-target "rearWheel")
        (range (start 183 23) (end 183 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel") (range (start 171 4) (end 171 37)))
        )
      )
    )
    (query (range (start 125 20) (end 125 30)) (probe (position 125 20))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))
        (kind featureTyping) (ordinal 0) (authored-target "Driveshaft")
        (range (start 125 20) (end 125 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft") (range (start 61 2) (end 61 96)))
        )
      )
    )
    (query (range (start 1 15) (end 1 26)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 1 15) (end 1 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions") (range (start 4 1) (end 4 2107)))
        )
      )
    )
    (query (range (start 89 18) (end 89 29)) (probe (position 89 18))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft"))
        (kind featureTyping) (ordinal 0) (authored-target "Driveshaft")
        (range (start 89 18) (end 89 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft") (range (start 61 2) (end 61 96)))
        )
      )
    )
    (query (range (start 96 11) (end 96 22)) (probe (position 96 11))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))
        (kind connectionSource) (ordinal 0) (authored-target "shaftPort_a")
        (range (start 96 11) (end 96 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a") (range (start 86 3) (end 86 32)))
        )
      )
    )
    (query (range (start 102 37) (end 102 48)) (probe (position 102 37))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))
        (kind connectionTarget) (ordinal 1) (authored-target "shaftPort_d")
        (range (start 102 37) (end 102 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d") (range (start 87 3) (end 87 32)))
        )
      )
    )
    (query (range (start 111 8) (end 111 19)) (probe (position 111 8))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind bindSource) (ordinal 0) (authored-target "fuelCmdPort")
        (range (start 111 8) (end 111 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 143 9) (end 143 20)) (probe (position 143 9))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind bindSource) (ordinal 0) (authored-target "shaftPort_d")
        (range (start 143 9) (end 143 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 39 31) (end 39 43)) (probe (position 39 31))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))
        (kind specialization) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 39 31) (end 39 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly") (range (start 38 2) (end 38 24)))
        )
      )
    )
    (query (range (start 123 22) (end 123 34)) (probe (position 123 22))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 123 22) (end 123 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission") (range (start 56 2) (end 56 96)))
        )
      )
    )
    (query (range (start 145 23) (end 145 35)) (probe (position 145 23))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))
        (kind featureTyping) (ordinal 0) (authored-target "Differential")
        (range (start 145 23) (end 145 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential") (range (start 66 2) (end 66 144)))
        )
      )
    )
    (query (range (start 197 32) (end 197 47)) (probe (position 197 32))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))
        (kind subsetting) (ordinal 0) (authored-target "wheelToRoadPort")
        (range (start 197 32) (end 197 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 198 33) (end 198 48)) (probe (position 198 33))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))
        (kind subsetting) (ordinal 0) (authored-target "wheelToRoadPort")
        (range (start 198 33) (end 198 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 142 26) (end 142 42)) (probe (position 142 26))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "RearAxleAssembly")
        (range (start 142 26) (end 142 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly") (range (start 39 2) (end 39 84)))
        )
      )
    )
    (query (range (start 111 22) (end 111 40)) (probe (position 111 22))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind bindTarget) (ordinal 0) (authored-target "engine::fuelCmdPort")
        (range (start 111 22) (end 111 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 116 12) (end 116 31)) (probe (position 116 12))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind connectionSource) (ordinal 1) (authored-target "engine::drivePwrPort")
        (range (start 116 12) (end 116 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 96 26) (end 96 48)) (probe (position 96 26))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))
        (kind connectionTarget) (ordinal 0) (authored-target "driveshaft::shaftPort_b")
        (range (start 96 26) (end 96 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 102 11) (end 102 33)) (probe (position 102 11))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))
        (kind connectionSource) (ordinal 1) (authored-target "driveshaft::shaftPort_c")
        (range (start 102 11) (end 102 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 116 35) (end 116 58)) (probe (position 116 35))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind connectionTarget) (ordinal 1) (authored-target "transmission::clutchPort")
        (range (start 116 35) (end 116 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 133 12) (end 133 36)) (probe (position 133 12))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind connectionSource) (ordinal 2) (authored-target "transmission::shaftPort_a")
        (range (start 133 12) (end 133 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 143 23) (end 143 47)) (probe (position 143 23))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind bindTarget) (ordinal 0) (authored-target "differential::shaftPort_d")
        (range (start 143 23) (end 143 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d") (range (start 146 5) (end 146 172)))
        )
      )
    )
    (query (range (start 156 14) (end 156 39)) (probe (position 156 14))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind connectionSource) (ordinal 1) (authored-target "differential::leftDiffPort")
        (range (start 156 14) (end 156 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort") (range (start 152 5) (end 152 33)))
        )
      )
    )
    (query (range (start 168 53) (end 168 78)) (probe (position 168 53))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind connectionTarget) (ordinal 3) (authored-target "leftWheel::wheelToAxlePort")
        (range (start 168 53) (end 168 78))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort") (range (start 179 5) (end 179 43)))
        )
      )
    )
    (query (range (start 161 14) (end 161 40)) (probe (position 161 14))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind connectionSource) (ordinal 2) (authored-target "differential::rightDiffPort")
        (range (start 161 14) (end 161 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort") (range (start 153 5) (end 153 34)))
        )
      )
    )
    (query (range (start 169 54) (end 169 80)) (probe (position 169 54))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind connectionTarget) (ordinal 4) (authored-target "rightWheel::wheelToAxlePort")
        (range (start 169 54) (end 169 80))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort") (range (start 184 5) (end 184 43)))
        )
      )
    )
    (query (range (start 196 36) (end 196 63)) (probe (position 196 36))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))
        (kind redefinition) (ordinal 0) (authored-target "VehicleA::vehicleToRoadPort")
        (range (start 196 36) (end 196 63))
        (outcome (status unresolved))
      )
    )
    (query (range (start 133 40) (end 133 68)) (probe (position 133 40))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind connectionTarget) (ordinal 2) (authored-target "rearAxleAssembly::shaftPort_d")
        (range (start 133 40) (end 133 68))
        (outcome (status unresolved))
      )
    )
    (query (range (start 156 43) (end 156 79)) (probe (position 156 43))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind connectionTarget) (ordinal 1) (authored-target "rearAxle::leftHalfAxle::axleToDiffPort")
        (range (start 156 43) (end 156 79))
        (outcome (status unresolved))
      )
    )
    (query (range (start 161 44) (end 161 81)) (probe (position 161 44))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind connectionTarget) (ordinal 2) (authored-target "rearAxle::rightHalfAxle::axleToDiffPort")
        (range (start 161 44) (end 161 81))
        (outcome (status unresolved))
      )
    )
    (query (range (start 168 12) (end 168 49)) (probe (position 168 12))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind connectionSource) (ordinal 3) (authored-target "rearAxle::leftHalfAxle::axleToWheelPort")
        (range (start 168 12) (end 168 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 191 5) (end 191 42)) (probe (position 191 5))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind bindTarget) (ordinal 3) (authored-target "vehicleToRoadPort::leftWheelToRoadPort")
        (range (start 191 5) (end 191 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort") (range (start 197 4) (end 197 70)))
        )
      )
    )
    (query (range (start 169 12) (end 169 50)) (probe (position 169 12))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))
        (kind connectionSource) (ordinal 4) (authored-target "rearAxle::rightHalfAxle::axleToWheelPort")
        (range (start 169 12) (end 169 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 194 5) (end 194 43)) (probe (position 194 5))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind bindTarget) (ordinal 4) (authored-target "vehicleToRoadPort::rightWheelToRoadPort")
        (range (start 194 5) (end 194 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort") (range (start 198 4) (end 198 71)))
        )
      )
    )
    (query (range (start 190 8) (end 190 50)) (probe (position 190 8))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind bindSource) (ordinal 3) (authored-target "rearAxleAssembly::leftWheel::wheelToRoadPort")
        (range (start 190 8) (end 190 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort") (range (start 180 5) (end 180 43)))
        )
      )
    )
    (query (range (start 193 8) (end 193 51)) (probe (position 193 8))
      (reference
        (source (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))
        (kind bindSource) (ordinal 4) (authored-target "rearAxleAssembly::rightWheel::wheelToRoadPort")
        (range (start 193 8) (end 193 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort") (range (start 185 5) (end 185 43)))
        )
      )
    )
  )
)
~~~
