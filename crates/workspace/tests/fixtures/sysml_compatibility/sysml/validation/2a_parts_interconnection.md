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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
LineComment,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
RegularComment,
KwPort,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
LineComment,
KwInterface,KwDef,Ident,OpenCurly,
RegularComment,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
KwRef,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwConnect,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwInterface,Colon,Ident,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwInterface,Colon,Ident,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Eq,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwInterface,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwInterface,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,Semicolon,
RegularComment,
KwPart,Ident,ColonGt,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwBind,Ident,Dot,Ident,Dot,Ident,Eq,
Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Dot,Ident,Eq,
Ident,Dot,Ident,Semicolon,
KwPort,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwPort,Ident,ColonGt,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwPort,Ident,ColonGt,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''2a-Parts Interconnection''
    (import_decl public 'Definitions::*')
    (import_decl public 'Usages::*')
    (package_def 'Definitions'
      (line_comment)
      (port_def 'FuelCmdPort')
      (port_def 'DrivePwrPort')
      (port_def 'ClutchPort')
      (port_def 'ShaftPort_a')
      (port_def 'ShaftPort_b')
      (port_def 'ShaftPort_c')
      (port_def 'ShaftPort_d')
      (port_def 'DiffPort')
      (port_def 'AxlePort')
      (port_def 'AxleToWheelPort')
      (port_def 'WheelToAxlePort')
      (port_def 'WheelToRoadPort')
      (port_def 'VehicleToRoadPort'
        (comment)
        (port_usage 'wheelToRoadPort' : 'WheelToRoadPort' multiplicity))
      (line_comment)
      (part_def 'VehicleA'
        (port_usage 'fuelCmdPort' : 'FuelCmdPort')
        (port_usage 'vehicleToRoadPort' : 'VehicleToRoadPort'))
      (part_def 'AxleAssembly')
      (part_def 'RearAxleAssembly' :> 'AxleAssembly'
        (port_usage 'shaftPort_d' : 'ShaftPort_d'))
      (part_def 'Axle')
      (part_def 'RearAxle' :> 'Axle')
      (part_def 'HalfAxle'
        (port_usage 'axleToDiffPort' : 'AxlePort')
        (port_usage 'axleToWheelPort' : 'AxleToWheelPort'))
      (part_def 'Engine'
        (port_usage 'fuelCmdPort' : 'FuelCmdPort')
        (port_usage 'drivePwrPort' : 'DrivePwrPort'))
      (part_def 'Transmission'
        (port_usage 'clutchPort' : 'ClutchPort')
        (port_usage 'shaftPort_a' : 'ShaftPort_a'))
      (part_def 'Driveshaft'
        (port_usage 'shaftPort_b' : 'ShaftPort_b')
        (port_usage 'shaftPort_c' : 'ShaftPort_c'))
      (part_def 'Differential'
        (comment))
      (part_def 'Wheel')
      (line_comment)
      (interface_def 'EngineToTransmissionInterface'
        (comment)
        (interface_end end 'drivePwrPort' : 'DrivePwrPort')
        (interface_end end 'clutchPort' : 'ClutchPort'))
      (interface_def 'DriveshaftInterface'
        (interface_end end 'shaftPort_a' : 'ShaftPort_a')
        (interface_end end 'shaftPort_d' : 'ShaftPort_d')
        (ref_usage ref 'driveshaft' : 'Driveshaft'
          (comment))
        (connection_usage
          (connector_end)
          (connector_end)
          (comment))
        (connection_usage
          (connector_end)
          (connector_end))))
    (package_def 'Usages'
      (part_usage 'vehicle1_c1' : 'VehicleA'
        (binding_as_usage
          (connector_end)
          (connector_end))
        (part_usage 'engine' : 'Engine')
        (interface_usage 'EngineToTransmissionInterface'
          (connector_end)
          (connector_end)
          (comment))
        (part_usage 'transmission' : 'Transmission')
        (part_usage 'driveshaft' : 'Driveshaft'
          (comment))
        (interface_usage 'DriveshaftInterface'
          (connector_end)
          (connector_end)
          (ref_usage ref :>> 'driveshaft' value
            (comment)))
        (part_usage 'rearAxleAssembly' : 'RearAxleAssembly'
          (binding_as_usage
            (connector_end)
            (connector_end))
          (part_usage 'differential' : 'Differential'
            (port_usage 'shaftPort_d' : 'ShaftPort_d'
              (comment))
            (port_usage 'leftDiffPort' : 'DiffPort')
            (port_usage 'rightDiffPort' : 'DiffPort'))
          (malformed)
          (malformed)
          (part_usage 'rearAxle' : 'RearAxle'
            (part_usage 'leftHalfAxle' : 'HalfAxle')
            (part_usage 'rightHalfAxle' : 'HalfAxle'))
          (connection_usage
            (connector_end)
            (connector_end))
          (connection_usage
            (connector_end)
            (connector_end))
          (part_usage 'rearWheel' : 'Wheel' multiplicity ordered)
          (comment)
          (part_usage 'leftWheel' :> 'rearWheel' value
            (port_usage 'wheelToAxlePort' : 'WheelToAxlePort')
            (port_usage 'wheelToRoadPort' : 'WheelToRoadPort'))
          (part_usage 'rightWheel' :> 'rearWheel' value
            (port_usage 'wheelToAxlePort' : 'WheelToAxlePort')
            (port_usage 'wheelToRoadPort' : 'WheelToRoadPort')))
        (binding_as_usage
          (connector_end)
          (connector_end))
        (binding_as_usage
          (connector_end)
          (connector_end))
        (port_usage 'vehicleToRoadPort' :>> 'VehicleA::vehicleToRoadPort'
          (port_usage 'leftWheelToRoadPort' :> 'wheelToRoadPort' value)
          (port_usage 'rightWheelToRoadPort' :> 'wheelToRoadPort' value))))))
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

            port wheelToRoadPort : WheelToRoadPort [2];
        }

        // Blocks

        part def VehicleA {
            port fuelCmdPort : FuelCmdPort;
            port vehicleToRoadPort : VehicleToRoadPort;
        }

        part def AxleAssembly;
        part def RearAxleAssembly :> AxleAssembly {
            port shaftPort_d : ShaftPort_d;
        }

        part def Axle;
        part def RearAxle :> Axle;

        part def HalfAxle {
            port axleToDiffPort : AxlePort;
            port axleToWheelPort : AxleToWheelPort;
        }

        part def Engine {
            port fuelCmdPort : FuelCmdPort;
            port drivePwrPort : DrivePwrPort;
        }

        part def Transmission {
            port clutchPort : ClutchPort;
            port shaftPort_a : ShaftPort_a;
        }

        part def Driveshaft {
            port shaftPort_b : ShaftPort_b;
            port shaftPort_c : ShaftPort_c;
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

            end drivePwrPort : DrivePwrPort;
            end clutchPort : ClutchPort;
        }

        interface def DriveshaftInterface {
            end shaftPort_a : ShaftPort_a;
            end shaftPort_d : ShaftPort_d;

            ref driveshaft : Driveshaft {
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
        part vehicle1_c1 : VehicleA {
            bind fuelCmdPort = engine.fuelCmdPort;

            part engine : Engine;

            interface : EngineToTransmissionInterface connect engine.drivePwrPort to transmission.clutchPort {
                /*
				 * A usage of an interface definition connects two ports relative to 
				 * a containing context.
				 */
            }

            part transmission : Transmission;

            part driveshaft : Driveshaft {
                /*
				 * This 'driveshaft' is the part of 'vehicle1_c1' that will act as the
				 * interface medium in the following 'DriveshaftInterface' usage.
				 */
            }

            interface : DriveshaftInterface connect transmission.shaftPort_a to rearAxleAssembly.shaftPort_d {
                ref :>> driveshaft = vehicle1_c1.driveshaft {
                    /*
						 * The reference property from 'DriveshaftInterface' is redefined
						 * in order to bind it to the appropriate part of 'vehicle1_c1'.
						 */
                }
            }

            part rearAxleAssembly : RearAxleAssembly {
                bind shaftPort_d = differential.shaftPort_d;

                part differential : Differential {
                    port shaftPort_d : ShaftPort_d {
                        /*
						 * If the part def has no ports, then they can be defined directly in
						 * a usage of the part def.
						 */
                    }
                    port leftDiffPort : DiffPort;
                    port rightDiffPort : DiffPort;
                }

                .leftDiffPort to rearAxle.leftHalfAxle.axleToDiffPort {
					/*
					 * A connection can be to a port that is arbitrarily deeply nested, on either end. 
					 */
				}
                .rightDiffPort to rearAxle.rightHalfAxle.axleToDiffPort;

                part rearAxle : RearAxle {
                    part leftHalfAxle : HalfAxle;
                    part rightHalfAxle : HalfAxle;
                }

                connect rearAxle.leftHalfAxle.axleToWheelPort to leftWheel.wheelToAxlePort;
                connect rearAxle.rightHalfAxle.axleToWheelPort to rightWheel.wheelToAxlePort;

                part rearWheel : Wheel [2] ordered;

                /* The two rear wheels of 'rearAxleAssembly' must be given
				 * their own names in order to be referenced in connections.
				 * 
				 * (":>" is a shorthand here for "subsets".)
				 */
                part leftWheel :> rearWheel = rearWheel#(1) {
                    port wheelToAxlePort : WheelToAxlePort;
                    port wheelToRoadPort : WheelToRoadPort;
                }

                part rightWheel :> rearWheel = rearWheel#(2) {
                    port wheelToAxlePort : WheelToAxlePort;
                    port wheelToRoadPort : WheelToRoadPort;
                }
            }

            bind rearAxleAssembly.leftWheel.wheelToRoadPort = vehicleToRoadPort.leftWheelToRoadPort;

            bind rearAxleAssembly.rightWheel.wheelToRoadPort = vehicleToRoadPort.rightWheelToRoadPort;

            port vehicleToRoadPort redefines VehicleA::vehicleToRoadPort {
                port leftWheelToRoadPort :> wheelToRoadPort = wheelToRoadPort#(1);
                port rightWheelToRoadPort :> wheelToRoadPort = wheelToRoadPort#(2);
            }
        }
    }
}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "2a-Parts Interconnection"))) (name "2a-Parts Interconnection") (declared-name "2a-Parts Interconnection")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Axle"))) (name "Axle") (declared-name "Axle") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly"))) (name "AxleAssembly") (declared-name "AxleAssembly") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))) (name "AxlePort") (declared-name "AxlePort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort::~AxlePort"))) (name "~AxlePort") (declared-name "~AxlePort") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))) (name "AxleToWheelPort") (declared-name "AxleToWheelPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort::~AxleToWheelPort"))) (name "~AxleToWheelPort") (declared-name "~AxleToWheelPort") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))) (name "ClutchPort") (declared-name "ClutchPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort::~ClutchPort"))) (name "~ClutchPort") (declared-name "~ClutchPort") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))) (name "DiffPort") (declared-name "DiffPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort::~DiffPort"))) (name "~DiffPort") (declared-name "~DiffPort") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential"))) (name "Differential") (declared-name "Differential") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))) (name "DrivePwrPort") (declared-name "DrivePwrPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort::~DrivePwrPort"))) (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))) (name "Driveshaft") (declared-name "Driveshaft") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (name "shaftPort_b") (declared-name "shaftPort_b") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (name "shaftPort_c") (declared-name "shaftPort_c") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft")))))
              )
            )
            (element (kind "interface def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface"))) (name "DriveshaftInterface") (declared-name "DriveshaftInterface")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft"))) (name "driveshaft") (declared-name "driveshaft") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a"))) (name "shaftPort_a") (declared-name "shaftPort_a") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d"))) (name "shaftPort_d") (declared-name "shaftPort_d") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))) (name "Engine") (declared-name "Engine") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (name "drivePwrPort") (declared-name "drivePwrPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine")))))
              )
            )
            (element (kind "interface def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface"))) (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface")
              (contains
                (element (kind "interface end") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::drivePwrPort"))) (name "drivePwrPort") (declared-name "drivePwrPort") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))) (name "FuelCmdPort") (declared-name "FuelCmdPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort::~FuelCmdPort"))) (name "~FuelCmdPort") (declared-name "~FuelCmdPort") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))) (name "HalfAxle") (declared-name "HalfAxle") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (name "axleToDiffPort") (declared-name "axleToDiffPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (name "axleToWheelPort") (declared-name "axleToWheelPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (name "RearAxle") (declared-name "RearAxle") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (name "RearAxleAssembly") (declared-name "RearAxleAssembly") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (name "shaftPort_d") (declared-name "shaftPort_d") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))) (name "ShaftPort_a") (declared-name "ShaftPort_a")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a::~ShaftPort_a"))) (name "~ShaftPort_a") (declared-name "~ShaftPort_a") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))) (name "ShaftPort_b") (declared-name "ShaftPort_b")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b::~ShaftPort_b"))) (name "~ShaftPort_b") (declared-name "~ShaftPort_b") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))) (name "ShaftPort_c") (declared-name "ShaftPort_c")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c::~ShaftPort_c"))) (name "~ShaftPort_c") (declared-name "~ShaftPort_c") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))) (name "ShaftPort_d") (declared-name "ShaftPort_d")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d::~ShaftPort_d"))) (name "~ShaftPort_d") (declared-name "~ShaftPort_d") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (name "shaftPort_a") (declared-name "shaftPort_a") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))) (name "VehicleA") (declared-name "VehicleA") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))) (name "VehicleToRoadPort") (declared-name "VehicleToRoadPort")
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (declared (properties (composite true) (reference false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort")))))
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::~VehicleToRoadPort"))) (name "~VehicleToRoadPort") (declared-name "~VehicleToRoadPort") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))) (name "WheelToAxlePort") (declared-name "WheelToAxlePort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort::~WheelToAxlePort"))) (name "~WheelToAxlePort") (declared-name "~WheelToAxlePort") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))) (name "WheelToRoadPort") (declared-name "WheelToRoadPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort::~WheelToRoadPort"))) (name "~WheelToRoadPort") (declared-name "~WheelToRoadPort") (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (name "vehicle1_c1") (declared-name "vehicle1_c1") (declared (properties (composite true) (reference false) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (name "driveshaft") (declared-name "driveshaft") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (name "differential") (declared-name "differential") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (name "leftDiffPort") (declared-name "leftDiffPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential")))))
                        (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (name "rightDiffPort") (declared-name "rightDiffPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential")))))
                        (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (name "shaftPort_d") (declared-name "shaftPort_d") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential")))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (name "leftWheel") (declared-name "leftWheel") (declared (properties (composite true) (reference false) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (name "wheelToAxlePort") (declared-name "wheelToAxlePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
                        (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared-name "rearAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (name "leftHalfAxle") (declared-name "leftHalfAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle")))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (name "rightHalfAxle") (declared-name "rightHalfAxle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle")))))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (name "rearWheel") (declared-name "rearWheel") (declared (properties (composite true) (reference false) (ordered true)) (multiplicity (lower 2) (upper 2) (ordered true) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (name "rightWheel") (declared-name "rightWheel") (declared (properties (composite true) (reference false) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (name "wheelToAxlePort") (declared-name "wheelToAxlePort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
                        (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly")))))
                      )
                    )
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (name "leftWheelToRoadPort") (declared-name "leftWheelToRoadPort") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (name "rightWheelToRoadPort") (declared-name "rightWheelToRoadPort") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA")))))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (bind (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (connect (source-expression "shaftPort_d") (target-expression "differential::shaftPort_d") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (connect (source-expression "fuelCmdPort") (target-expression "engine::fuelCmdPort") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (connect (source-expression "rearAxleAssembly::leftWheel::wheelToRoadPort") (target-expression "vehicleToRoadPort::leftWheelToRoadPort") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (connect (source-expression "rearAxleAssembly::rightWheel::wheelToRoadPort") (target-expression "vehicleToRoadPort::rightWheelToRoadPort") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d"))) (connect (source-expression "driveshaft::shaftPort_c") (target-expression "shaftPort_d") (container-prefix "2a-Parts Interconnection::Definitions::DriveshaftInterface")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (connect (source-expression "shaftPort_a") (target-expression "driveshaft::shaftPort_b") (container-prefix "2a-Parts Interconnection::Definitions::DriveshaftInterface")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (connect (source-expression "engine::drivePwrPort") (target-expression "transmission::clutchPort") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1") (interface-usage true) (interface-type "EngineToTransmissionInterface")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (connect (source-expression "rearAxle::leftHalfAxle::axleToWheelPort") (target-expression "leftWheel::wheelToAxlePort") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (connect (source-expression "rearAxle::rightHalfAxle::axleToWheelPort") (target-expression "rightWheel::wheelToAxlePort") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (connect (source-expression "transmission::shaftPort_a") (target-expression "rearAxleAssembly::shaftPort_d") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1") (interface-usage true) (interface-type "DriveshaftInterface")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (connect (source-expression "differential::leftDiffPort") (target-expression "rearAxle::leftHalfAxle::axleToDiffPort") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly") (interface-usage true)))
    (connection (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (connect (source-expression "differential::rightDiffPort") (target-expression "rearAxle::rightHalfAxle::axleToDiffPort") (container-prefix "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly") (interface-usage true)))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort::~AxlePort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort::~AxleToWheelPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort::~ClutchPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort::~DiffPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort::~DrivePwrPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort::~FuelCmdPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a::~ShaftPort_a"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b::~ShaftPort_b"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c::~ShaftPort_c"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d::~ShaftPort_d"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::~VehicleToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort::~WheelToAxlePort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort::~WheelToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Axle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleAssembly"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::leftWheelToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::vehicleToRoadPort::rightWheelToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_b"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_b"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft::shaftPort_c"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_c"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_a"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DriveshaftInterface::shaftPort_d"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::drivePwrPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine::fuelCmdPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::clutchPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::EngineToTransmissionInterface::drivePwrPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToDiffPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxlePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle::axleToWheelPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::AxleToWheelPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly::shaftPort_d"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::clutchPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ClutchPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission::shaftPort_a"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_a"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::fuelCmdPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::FuelCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::VehicleA"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::driveshaft"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Driveshaft"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::engine"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxleAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Differential"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::leftDiffPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::rightDiffPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::DiffPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::differential::shaftPort_d"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::ShaftPort_d"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToAxlePort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::leftWheel::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::RearAxle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::leftHalfAxle"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearAxle::rightHalfAxle"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::HalfAxle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToAxlePort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToAxlePort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rightWheel::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::WheelToRoadPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "2a-Parts Interconnection::Usages::vehicle1_c1::transmission"))) (to (node (document "d0") (qualified-name "2a-Parts Interconnection::Definitions::Transmission"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
