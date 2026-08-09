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
(model
  (namespace
    (package '2a-Parts Interconnection'
      (namespace_import public -> '2a-Parts Interconnection::Definitions'[package])
      (namespace_import public -> '2a-Parts Interconnection::Usages'[package])
      (package 'Definitions'
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
          (port_usage composite 'wheelToRoadPort' : '2a-Parts Interconnection::Definitions::WheelToRoadPort'[port_def]
            (multiplicity_range [2])))
        (part_def 'VehicleA'
          (port_usage composite 'fuelCmdPort' : '2a-Parts Interconnection::Definitions::FuelCmdPort'[port_def])
          (port_usage composite 'vehicleToRoadPort' : '2a-Parts Interconnection::Definitions::VehicleToRoadPort'[port_def]))
        (part_def 'AxleAssembly')
        (part_def 'RearAxleAssembly' :> '2a-Parts Interconnection::Definitions::AxleAssembly'[part_def]
          (port_usage composite 'shaftPort_d' : '2a-Parts Interconnection::Definitions::ShaftPort_d'[port_def]))
        (part_def 'Axle')
        (part_def 'RearAxle' :> '2a-Parts Interconnection::Definitions::Axle'[part_def])
        (part_def 'HalfAxle'
          (port_usage composite 'axleToDiffPort' : '2a-Parts Interconnection::Definitions::AxlePort'[port_def])
          (port_usage composite 'axleToWheelPort' : '2a-Parts Interconnection::Definitions::AxleToWheelPort'[port_def]))
        (part_def 'Engine'
          (port_usage composite 'fuelCmdPort' : '2a-Parts Interconnection::Definitions::FuelCmdPort'[port_def])
          (port_usage composite 'drivePwrPort' : '2a-Parts Interconnection::Definitions::DrivePwrPort'[port_def]))
        (part_def 'Transmission'
          (port_usage composite 'clutchPort' : '2a-Parts Interconnection::Definitions::ClutchPort'[port_def])
          (port_usage composite 'shaftPort_a' : '2a-Parts Interconnection::Definitions::ShaftPort_a'[port_def]))
        (part_def 'Driveshaft'
          (port_usage composite 'shaftPort_b' : '2a-Parts Interconnection::Definitions::ShaftPort_b'[port_def])
          (port_usage composite 'shaftPort_c' : '2a-Parts Interconnection::Definitions::ShaftPort_c'[port_def]))
        (part_def 'Differential')
        (part_def 'Wheel')
        (interface_def 'EngineToTransmissionInterface'
          (port_usage end 'drivePwrPort' : '2a-Parts Interconnection::Definitions::DrivePwrPort'[port_def])
          (port_usage end 'clutchPort' : '2a-Parts Interconnection::Definitions::ClutchPort'[port_def]))
        (interface_def 'DriveshaftInterface'
          (port_usage end 'shaftPort_a' : '2a-Parts Interconnection::Definitions::ShaftPort_a'[port_def])
          (port_usage end 'shaftPort_d' : '2a-Parts Interconnection::Definitions::ShaftPort_d'[port_def])
          (reference_usage reference 'driveshaft' : '2a-Parts Interconnection::Definitions::Driveshaft'[part_def])
          (connection_usage composite
            (connector_end 'shaftPort_a')
            (connector_end 'driveshaft.shaftPort_b'))
          (connection_usage composite
            (connector_end 'driveshaft.shaftPort_c')
            (connector_end 'shaftPort_d'))))
      (package 'Usages'
        (part_usage 'vehicle1_c1' : '2a-Parts Interconnection::Definitions::VehicleA'[part_def]
          (binding_connector_def
            (connector_end 'fuelCmdPort')
            (connector_end 'engine.fuelCmdPort'))
          (part_usage composite 'engine' : '2a-Parts Interconnection::Definitions::Engine'[part_def])
          (interface_usage composite : '2a-Parts Interconnection::Definitions::EngineToTransmissionInterface'[interface_def]
            (connector_end 'engine.drivePwrPort')
            (connector_end 'transmission.clutchPort'))
          (part_usage composite 'transmission' : '2a-Parts Interconnection::Definitions::Transmission'[part_def])
          (part_usage composite 'driveshaft' : '2a-Parts Interconnection::Definitions::Driveshaft'[part_def])
          (interface_usage composite : '2a-Parts Interconnection::Definitions::DriveshaftInterface'[interface_def]
            (connector_end 'transmission.shaftPort_a')
            (connector_end 'rearAxleAssembly.shaftPort_d')
            (reference_usage reference :>> '2a-Parts Interconnection::Definitions::DriveshaftInterface::driveshaft'[reference_usage]
              (feature_value (=))))
          (part_usage composite 'rearAxleAssembly' : '2a-Parts Interconnection::Definitions::RearAxleAssembly'[part_def]
            (binding_connector_def
              (connector_end 'shaftPort_d')
              (connector_end 'differential.shaftPort_d'))
            (part_usage composite 'differential' : '2a-Parts Interconnection::Definitions::Differential'[part_def]
              (port_usage composite 'shaftPort_d' : '2a-Parts Interconnection::Definitions::ShaftPort_d'[port_def])
              (port_usage composite 'leftDiffPort' : '2a-Parts Interconnection::Definitions::DiffPort'[port_def])
              (port_usage composite 'rightDiffPort' : '2a-Parts Interconnection::Definitions::DiffPort'[port_def]))
            (not_implemented 'malformed')
            (not_implemented 'malformed')
            (part_usage composite 'rearAxle' : '2a-Parts Interconnection::Definitions::RearAxle'[part_def]
              (part_usage composite 'leftHalfAxle' : '2a-Parts Interconnection::Definitions::HalfAxle'[part_def])
              (part_usage composite 'rightHalfAxle' : '2a-Parts Interconnection::Definitions::HalfAxle'[part_def]))
            (connection_usage composite
              (connector_end 'rearAxle.leftHalfAxle.axleToWheelPort')
              (connector_end 'leftWheel.wheelToAxlePort'))
            (connection_usage composite
              (connector_end 'rearAxle.rightHalfAxle.axleToWheelPort')
              (connector_end 'rightWheel.wheelToAxlePort'))
            (part_usage composite ordered 'rearWheel' : '2a-Parts Interconnection::Definitions::Wheel'[part_def]
              (multiplicity_range [2]))
            (part_usage composite 'leftWheel' :> '2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel'[part_usage]
              (feature_value (=))
              (port_usage composite 'wheelToAxlePort' : '2a-Parts Interconnection::Definitions::WheelToAxlePort'[port_def])
              (port_usage composite 'wheelToRoadPort' : '2a-Parts Interconnection::Definitions::WheelToRoadPort'[port_def]))
            (part_usage composite 'rightWheel' :> '2a-Parts Interconnection::Usages::vehicle1_c1::rearAxleAssembly::rearWheel'[part_usage]
              (feature_value (=))
              (port_usage composite 'wheelToAxlePort' : '2a-Parts Interconnection::Definitions::WheelToAxlePort'[port_def])
              (port_usage composite 'wheelToRoadPort' : '2a-Parts Interconnection::Definitions::WheelToRoadPort'[port_def])))
          (binding_connector_def
            (connector_end 'rearAxleAssembly.leftWheel.wheelToRoadPort')
            (connector_end 'vehicleToRoadPort.leftWheelToRoadPort'))
          (binding_connector_def
            (connector_end 'rearAxleAssembly.rightWheel.wheelToRoadPort')
            (connector_end 'vehicleToRoadPort.rightWheelToRoadPort'))
          (port_usage composite 'vehicleToRoadPort' :>> '2a-Parts Interconnection::Definitions::VehicleA::vehicleToRoadPort'[port_usage]
            (port_usage composite 'leftWheelToRoadPort' :> '2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort'[port_usage]
              (feature_value (=)))
            (port_usage composite 'rightWheelToRoadPort' :> '2a-Parts Interconnection::Definitions::VehicleToRoadPort::wheelToRoadPort'[port_usage]
              (feature_value (=)))))))))
~~~
