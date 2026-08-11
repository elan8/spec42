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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f7603000b796ef6ea75526c50e4466c5ff658b0ad75325de723d69a52c7e5585") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection"))) (kind "package") (name "2a-Parts Interconnection") (declared-name "2a-Parts Interconnection") (range (start (line 0) (character 0)) (end (line 0) (character 5072))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 30))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 26))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 25))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection"))) (authored (membership (kind Import) (visibility "public") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 15)) (end (line 2) (character 21))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 4) (character 1)) (end (line 4) (character 2107))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Axle"))) (kind "part def") (name "Axle") (declared-name "Axle") (range (start (line 43) (character 2)) (end (line 43) (character 16))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly"))) (kind "part def") (name "AxleAssembly") (declared-name "AxleAssembly") (range (start (line 38) (character 2)) (end (line 38) (character 24))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))) (kind "port def") (name "AxlePort") (declared-name "AxlePort") (range (start (line 18) (character 2)) (end (line 18) (character 20))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort::~AxlePort"))) (kind "conjugated port definition") (name "~AxlePort") (declared-name "~AxlePort") (range (start (line 18) (character 2)) (end (line 18) (character 20))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))) (kind "port def") (name "AxleToWheelPort") (declared-name "AxleToWheelPort") (range (start (line 19) (character 2)) (end (line 19) (character 27))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort::~AxleToWheelPort"))) (kind "conjugated port definition") (name "~AxleToWheelPort") (declared-name "~AxleToWheelPort") (range (start (line 19) (character 2)) (end (line 19) (character 27))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))) (kind "port def") (name "ClutchPort") (declared-name "ClutchPort") (range (start (line 10) (character 2)) (end (line 10) (character 22))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort::~ClutchPort"))) (kind "conjugated port definition") (name "~ClutchPort") (declared-name "~ClutchPort") (range (start (line 10) (character 2)) (end (line 10) (character 22))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))) (kind "port def") (name "DiffPort") (declared-name "DiffPort") (range (start (line 17) (character 2)) (end (line 17) (character 20))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort::~DiffPort"))) (kind "conjugated port definition") (name "~DiffPort") (declared-name "~DiffPort") (range (start (line 17) (character 2)) (end (line 17) (character 20))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential"))) (kind "part def") (name "Differential") (declared-name "Differential") (range (start (line 66) (character 2)) (end (line 66) (character 144))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))) (kind "port def") (name "DrivePwrPort") (declared-name "DrivePwrPort") (range (start (line 9) (character 2)) (end (line 9) (character 24))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort::~DrivePwrPort"))) (kind "conjugated port definition") (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (range (start (line 9) (character 2)) (end (line 9) (character 24))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (kind "part def") (name "Driveshaft") (declared-name "Driveshaft") (range (start (line 61) (character 2)) (end (line 61) (character 96))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (kind "port") (name "shaftPort_b") (declared-name "shaftPort_b") (range (start (line 62) (character 3)) (end (line 62) (character 33))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_b") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (kind "port") (name "shaftPort_c") (declared-name "shaftPort_c") (range (start (line 63) (character 3)) (end (line 63) (character 33))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_c") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind "interface def") (name "DriveshaftInterface") (declared-name "DriveshaftInterface") (range (start (line 85) (character 2)) (end (line 85) (character 514))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft"))) (kind "ref") (name "driveshaft") (declared-name "driveshaft") (range (start (line 89) (character 3)) (end (line 89) (character 171))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driveshaft") (range (start (line 89) (character 18)) (end (line 89) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a"))) (kind "interface end") (name "shaftPort_a") (declared-name "shaftPort_a") (range (start (line 86) (character 3)) (end (line 86) (character 32))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (authored (relationships (typing (reference "ShaftPort_a") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d"))) (kind "interface end") (name "shaftPort_d") (declared-name "shaftPort_d") (range (start (line 87) (character 3)) (end (line 87) (character 32))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (authored (relationships (typing (reference "ShaftPort_d") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 51) (character 2)) (end (line 51) (character 94))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (range (start (line 53) (character 3)) (end (line 53) (character 35))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "DrivePwrPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (range (start (line 52) (character 3)) (end (line 52) (character 33))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelCmdPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface"))) (kind "interface def") (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface") (range (start (line 76) (character 2)) (end (line 76) (character 193))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::clutchPort"))) (kind "interface end") (name "clutchPort") (declared-name "clutchPort") (range (start (line 82) (character 3)) (end (line 82) (character 30))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "ClutchPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind "interface end") (name "drivePwrPort") (declared-name "drivePwrPort") (range (start (line 81) (character 3)) (end (line 81) (character 34))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "DrivePwrPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))) (kind "port def") (name "FuelCmdPort") (declared-name "FuelCmdPort") (range (start (line 7) (character 2)) (end (line 7) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort::~FuelCmdPort"))) (kind "conjugated port definition") (name "~FuelCmdPort") (declared-name "~FuelCmdPort") (range (start (line 7) (character 2)) (end (line 7) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (kind "part def") (name "HalfAxle") (declared-name "HalfAxle") (range (start (line 46) (character 2)) (end (line 46) (character 102))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (kind "port") (name "axleToDiffPort") (declared-name "axleToDiffPort") (range (start (line 47) (character 3)) (end (line 47) (character 33))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxlePort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (kind "port") (name "axleToWheelPort") (declared-name "axleToWheelPort") (range (start (line 48) (character 3)) (end (line 48) (character 41))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleToWheelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (kind "part def") (name "RearAxle") (declared-name "RearAxle") (range (start (line 44) (character 2)) (end (line 44) (character 28))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Axle") (range (start (line 44) (character 23)) (end (line 44) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (kind "part def") (name "RearAxleAssembly") (declared-name "RearAxleAssembly") (range (start (line 39) (character 2)) (end (line 39) (character 84))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AxleAssembly") (range (start (line 39) (character 31)) (end (line 39) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (kind "port") (name "shaftPort_d") (declared-name "shaftPort_d") (range (start (line 40) (character 3)) (end (line 40) (character 33))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_d") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))) (kind "port def") (name "ShaftPort_a") (declared-name "ShaftPort_a") (range (start (line 12) (character 2)) (end (line 12) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a::~ShaftPort_a"))) (kind "conjugated port definition") (name "~ShaftPort_a") (declared-name "~ShaftPort_a") (range (start (line 12) (character 2)) (end (line 12) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))) (kind "port def") (name "ShaftPort_b") (declared-name "ShaftPort_b") (range (start (line 13) (character 2)) (end (line 13) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b::~ShaftPort_b"))) (kind "conjugated port definition") (name "~ShaftPort_b") (declared-name "~ShaftPort_b") (range (start (line 13) (character 2)) (end (line 13) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))) (kind "port def") (name "ShaftPort_c") (declared-name "ShaftPort_c") (range (start (line 14) (character 2)) (end (line 14) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c::~ShaftPort_c"))) (kind "conjugated port definition") (name "~ShaftPort_c") (declared-name "~ShaftPort_c") (range (start (line 14) (character 2)) (end (line 14) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))) (kind "port def") (name "ShaftPort_d") (declared-name "ShaftPort_d") (range (start (line 15) (character 2)) (end (line 15) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d::~ShaftPort_d"))) (kind "conjugated port definition") (name "~ShaftPort_d") (declared-name "~ShaftPort_d") (range (start (line 15) (character 2)) (end (line 15) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 56) (character 2)) (end (line 56) (character 96))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 57) (character 3)) (end (line 57) (character 31))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClutchPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (kind "port") (name "shaftPort_a") (declared-name "shaftPort_a") (range (start (line 58) (character 3)) (end (line 58) (character 33))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_a") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (kind "part def") (name "VehicleA") (declared-name "VehicleA") (range (start (line 33) (character 2)) (end (line 33) (character 106))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (range (start (line 34) (character 3)) (end (line 34) (character 33))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelCmdPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (kind "port") (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (range (start (line 35) (character 3)) (end (line 35) (character 45))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleToRoadPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))) (kind "port def") (name "VehicleToRoadPort") (declared-name "VehicleToRoadPort") (range (start (line 23) (character 2)) (end (line 23) (character 143))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (range (start (line 28) (character 3)) (end (line 28) (character 44))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::~VehicleToRoadPort"))) (kind "conjugated port definition") (name "~VehicleToRoadPort") (declared-name "~VehicleToRoadPort") (range (start (line 23) (character 2)) (end (line 23) (character 143))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 72) (character 2)) (end (line 72) (character 17))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))) (kind "port def") (name "WheelToAxlePort") (declared-name "WheelToAxlePort") (range (start (line 20) (character 2)) (end (line 20) (character 27))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort::~WheelToAxlePort"))) (kind "conjugated port definition") (name "~WheelToAxlePort") (declared-name "~WheelToAxlePort") (range (start (line 20) (character 2)) (end (line 20) (character 27))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (kind "port def") (name "WheelToRoadPort") (declared-name "WheelToRoadPort") (range (start (line 21) (character 2)) (end (line 21) (character 27))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort::~WheelToRoadPort"))) (kind "conjugated port definition") (name "~WheelToRoadPort") (declared-name "~WheelToRoadPort") (range (start (line 21) (character 2)) (end (line 21) (character 27))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 107) (character 1)) (end (line 107) (character 2863))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection"))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 109) (character 2)) (end (line 109) (character 2838))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA") (range (start (line 109) (character 20)) (end (line 109) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (kind "part") (name "driveshaft") (declared-name "driveshaft") (range (start (line 125) (character 3)) (end (line 125) (character 197))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driveshaft") (range (start (line 125) (character 20)) (end (line 125) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 113) (character 3)) (end (line 113) (character 23))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 113) (character 16)) (end (line 113) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 142) (character 3)) (end (line 142) (character 1483))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "RearAxleAssembly") (range (start (line 142) (character 26)) (end (line 142) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (kind "part") (name "differential") (declared-name "differential") (range (start (line 145) (character 4)) (end (line 145) (character 285))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Differential") (range (start (line 145) (character 23)) (end (line 145) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (kind "port") (name "leftDiffPort") (declared-name "leftDiffPort") (range (start (line 152) (character 5)) (end (line 152) (character 33))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (authored (membership (kind Feature)) (relationships (typing (reference "DiffPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (kind "port") (name "rightDiffPort") (declared-name "rightDiffPort") (range (start (line 153) (character 5)) (end (line 153) (character 34))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (authored (membership (kind Feature)) (relationships (typing (reference "DiffPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (kind "port") (name "shaftPort_d") (declared-name "shaftPort_d") (range (start (line 146) (character 5)) (end (line 146) (character 172))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_d") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (kind "part") (name "leftWheel") (declared-name "leftWheel") (range (start (line 178) (character 4)) (end (line 178) (character 143))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel") (range (start (line 178) (character 22)) (end (line 178) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (kind "port") (name "wheelToAxlePort") (declared-name "wheelToAxlePort") (range (start (line 179) (character 5)) (end (line 179) (character 43))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToAxlePort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (range (start (line 180) (character 5)) (end (line 180) (character 43))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (range (start (line 163) (character 4)) (end (line 163) (character 104))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "RearAxle") (range (start (line 163) (character 19)) (end (line 163) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind "part") (name "leftHalfAxle") (declared-name "leftHalfAxle") (range (start (line 164) (character 5)) (end (line 164) (character 33))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "HalfAxle") (range (start (line 164) (character 24)) (end (line 164) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind "part") (name "rightHalfAxle") (declared-name "rightHalfAxle") (range (start (line 165) (character 5)) (end (line 165) (character 34))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "HalfAxle") (range (start (line 165) (character 25)) (end (line 165) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (range (start (line 171) (character 4)) (end (line 171) (character 37))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 171) (character 20)) (end (line 171) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (kind "part") (name "rightWheel") (declared-name "rightWheel") (range (start (line 183) (character 4)) (end (line 183) (character 144))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel") (range (start (line 183) (character 23)) (end (line 183) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind "port") (name "wheelToAxlePort") (declared-name "wheelToAxlePort") (range (start (line 184) (character 5)) (end (line 184) (character 43))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToAxlePort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (range (start (line 185) (character 5)) (end (line 185) (character 43))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 123) (character 3)) (end (line 123) (character 35))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 123) (character 22)) (end (line 123) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (kind "port") (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (range (start (line 196) (character 3)) (end (line 196) (character 213))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleA::vehicleToRoadPort") (range (start (line 196) (character 36)) (end (line 196) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (kind "port") (name "leftWheelToRoadPort") (declared-name "leftWheelToRoadPort") (range (start (line 197) (character 4)) (end (line 197) (character 70))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "wheelToRoadPort") (range (start (line 197) (character 32)) (end (line 197) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (kind "port") (name "rightWheelToRoadPort") (declared-name "rightWheelToRoadPort") (range (start (line 198) (character 4)) (end (line 198) (character 71))) (parent (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "wheelToRoadPort") (range (start (line 198) (character 33)) (end (line 198) (character 48)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 1) (character 15)) (end (line 1) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (range (start (line 2) (character 15)) (end (line 2) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_b") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_c") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind connectionSource) (ordinal 0)) (authored-target "shaftPort_a") (range (start (line 96) (character 11)) (end (line 96) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind connectionSource) (ordinal 1)) (authored-target "driveshaft::shaftPort_c") (range (start (line 102) (character 11)) (end (line 102) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind connectionTarget) (ordinal 0)) (authored-target "driveshaft::shaftPort_b") (range (start (line 96) (character 26)) (end (line 96) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (kind connectionTarget) (ordinal 1)) (authored-target "shaftPort_d") (range (start (line 102) (character 37)) (end (line 102) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft"))) (kind featureTyping) (ordinal 0)) (authored-target "Driveshaft") (range (start (line 89) (character 18)) (end (line 89) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_a") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_d") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmdPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (kind featureTyping) (ordinal 0)) (authored-target "AxlePort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleToWheelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (kind specialization) (ordinal 0)) (authored-target "Axle") (range (start (line 44) (character 23)) (end (line 44) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Axle")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (kind specialization) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 39) (character 31)) (end (line 39) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_d") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_a") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmdPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleToRoadPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA") (range (start (line 109) (character 20)) (end (line 109) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind connectionSource) (ordinal 1)) (authored-target "engine::drivePwrPort") (range (start (line 116) (character 12)) (end (line 116) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind connectionSource) (ordinal 2)) (authored-target "transmission::shaftPort_a") (range (start (line 133) (character 12)) (end (line 133) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind connectionTarget) (ordinal 1)) (authored-target "transmission::clutchPort") (range (start (line 116) (character 35)) (end (line 116) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind connectionTarget) (ordinal 2)) (authored-target "rearAxleAssembly::shaftPort_d") (range (start (line 133) (character 40)) (end (line 133) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 0)) (authored-target "fuelCmdPort") (range (start (line 111) (character 8)) (end (line 111) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 3)) (authored-target "rearAxleAssembly::leftWheel::wheelToRoadPort") (range (start (line 190) (character 8)) (end (line 190) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 4)) (authored-target "rearAxleAssembly::rightWheel::wheelToRoadPort") (range (start (line 193) (character 8)) (end (line 193) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindTarget) (ordinal 0)) (authored-target "engine::fuelCmdPort") (range (start (line 111) (character 22)) (end (line 111) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindTarget) (ordinal 3)) (authored-target "vehicleToRoadPort::leftWheelToRoadPort") (range (start (line 191) (character 5)) (end (line 191) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindTarget) (ordinal 4)) (authored-target "vehicleToRoadPort::rightWheelToRoadPort") (range (start (line 194) (character 5)) (end (line 194) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (kind featureTyping) (ordinal 0)) (authored-target "Driveshaft") (range (start (line 125) (character 20)) (end (line 125) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 113) (character 16)) (end (line 113) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "RearAxleAssembly") (range (start (line 142) (character 26)) (end (line 142) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionSource) (ordinal 1)) (authored-target "differential::leftDiffPort") (range (start (line 156) (character 14)) (end (line 156) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionSource) (ordinal 2)) (authored-target "differential::rightDiffPort") (range (start (line 161) (character 14)) (end (line 161) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionSource) (ordinal 3)) (authored-target "rearAxle::leftHalfAxle::axleToWheelPort") (range (start (line 168) (character 12)) (end (line 168) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionSource) (ordinal 4)) (authored-target "rearAxle::rightHalfAxle::axleToWheelPort") (range (start (line 169) (character 12)) (end (line 169) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionTarget) (ordinal 1)) (authored-target "rearAxle::leftHalfAxle::axleToDiffPort") (range (start (line 156) (character 43)) (end (line 156) (character 79))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionTarget) (ordinal 2)) (authored-target "rearAxle::rightHalfAxle::axleToDiffPort") (range (start (line 161) (character 44)) (end (line 161) (character 81))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionTarget) (ordinal 3)) (authored-target "leftWheel::wheelToAxlePort") (range (start (line 168) (character 53)) (end (line 168) (character 78))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind connectionTarget) (ordinal 4)) (authored-target "rightWheel::wheelToAxlePort") (range (start (line 169) (character 54)) (end (line 169) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind bindSource) (ordinal 0)) (authored-target "shaftPort_d") (range (start (line 143) (character 9)) (end (line 143) (character 20))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (kind bindTarget) (ordinal 0)) (authored-target "differential::shaftPort_d") (range (start (line 143) (character 23)) (end (line 143) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (kind featureTyping) (ordinal 0)) (authored-target "Differential") (range (start (line 145) (character 23)) (end (line 145) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DiffPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DiffPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_d") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (range (start (line 178) (character 22)) (end (line 178) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToAxlePort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "RearAxle") (range (start (line 163) (character 19)) (end (line 163) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "HalfAxle") (range (start (line 164) (character 24)) (end (line 164) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "HalfAxle") (range (start (line 165) (character 25)) (end (line 165) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 171) (character 20)) (end (line 171) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (range (start (line 183) (character 23)) (end (line 183) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToAxlePort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 123) (character 22)) (end (line 123) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleA::vehicleToRoadPort") (range (start (line 196) (character 36)) (end (line 196) (character 63))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (kind subsetting) (ordinal 0)) (authored-target "wheelToRoadPort") (range (start (line 197) (character 32)) (end (line 197) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (kind subsetting) (ordinal 0)) (authored-target "wheelToRoadPort") (range (start (line 198) (character 33)) (end (line 198) (character 48))) (outcome (status unresolved)))
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
    (relationship (kind bind) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 3)) (expression (kind bind) (source "rearAxleAssembly::leftWheel::wheelToRoadPort") (target "vehicleToRoadPort::leftWheelToRoadPort") (source-range (start (line 190) (character 8)) (end (line 190) (character 50))) (target-range (start (line 191) (character 5)) (end (line 191) (character 42)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (kind bindSource) (ordinal 4)) (expression (kind bind) (source "rearAxleAssembly::rightWheel::wheelToRoadPort") (target "vehicleToRoadPort::rightWheelToRoadPort") (source-range (start (line 193) (character 8)) (end (line 193) (character 51))) (target-range (start (line 194) (character 5)) (end (line 194) (character 43)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (target (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
