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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
RegularComment,
KwPart,Ident,Colon,Ident,Semicolon,
KwFlow,KwOf,Ident,Colon,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFlow,KwOf,Ident,Colon,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
RegularComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Slash,Ident,Semicolon,
RegularComment,
KwItem,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''3d-Function-based Behavior-item''
    (import_decl private 'ScalarValues::Real')
    (import_decl public 'Definitions::*')
    (import_decl public 'Usages::*')
    (package_def 'Definitions'
      (item_def 'Fuel')
      (port_def 'FuelPort'
        (item_usage out 'fuel' : 'Fuel'))
      (part_def 'Pump'
        (port_usage 'fuelInPort' : ~'FuelPort')
        (port_usage 'fuelOutPort' : 'FuelPort'))
      (part_def 'StorageTank'
        (port_usage 'fuelOutPort' : 'FuelPort'))
      (part_def 'FuelTank'
        (port_usage 'fuelInPort' : ~'FuelPort'))
      (part_def 'Vehicle'
        (port_usage 'fuelInPort' : ~'FuelPort'))
      (action_def 'PumpFuel'
        (default_ref_usage in 'fuelIn' : 'Fuel')
        (default_ref_usage out 'fuelOut' : 'Fuel')))
    (package_def 'Usages'
      (part_usage 'context'
        (comment)
        (part_usage 'storageTank' : 'StorageTank')
        (flow_usage 'of'
          (comment))
        (part_usage 'pump' : 'Pump'
          (perform_action 'pumpFuel' : 'PumpFuel'
            (default_ref_usage in 'fuelIn' value)
            (default_ref_usage out 'fuelOut' value)))
        (flow_usage 'of')
        (part_usage 'vehicle' : 'Vehicle'
          (flow_usage 'fuelInPort'
            (comment))
          (comment)
          (part_usage 'fuelTank' : 'FuelTank'
            (attribute_usage 'volumeMax' : 'Real')
            (attribute_usage 'fuelLevel' : 'Real' value)
            (comment)
            (item_usage 'fuel' : 'Fuel'
              (attribute_usage 'volume' : 'Real')
              (comment))))))))
~~~
# FORMAT
~~~sysml
package '3d-Function-based Behavior-item' {
    private import ScalarValues::Real;
    public import Definitions::*;
    public import Usages::*;

    package Definitions {
        item def Fuel;

        port def FuelPort {
            out item fuel : Fuel;
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

            flow of {
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

            flow of;

            part vehicle : Vehicle {
                flow fuelInPort {
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
# EXPECTED
~~~
semantic.duplicate_name 'of'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'of'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(model
  (namespace
    (package '3d-Function-based Behavior-item'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import public -> '3d-Function-based Behavior-item::Definitions'[package])
      (namespace_import public -> '3d-Function-based Behavior-item::Usages'[package])
      (package 'Definitions'
        (item_def 'Fuel')
        (port_def 'FuelPort'
          (item_usage out 'fuel' : '3d-Function-based Behavior-item::Definitions::Fuel'[item_def]))
        (part_def 'Pump'
          (port_usage composite 'fuelInPort' : '3d-Function-based Behavior-item::Definitions::FuelPort'[port_def] ~ '3d-Function-based Behavior-item::Definitions::FuelPort'[port_def])
          (port_usage composite 'fuelOutPort' : '3d-Function-based Behavior-item::Definitions::FuelPort'[port_def]))
        (part_def 'StorageTank'
          (port_usage composite 'fuelOutPort' : '3d-Function-based Behavior-item::Definitions::FuelPort'[port_def]))
        (part_def 'FuelTank'
          (port_usage composite 'fuelInPort' : '3d-Function-based Behavior-item::Definitions::FuelPort'[port_def] ~ '3d-Function-based Behavior-item::Definitions::FuelPort'[port_def]))
        (part_def 'Vehicle'
          (port_usage composite 'fuelInPort' : '3d-Function-based Behavior-item::Definitions::FuelPort'[port_def] ~ '3d-Function-based Behavior-item::Definitions::FuelPort'[port_def]))
        (action_def 'PumpFuel'
          (reference_usage in reference 'fuelIn' : '3d-Function-based Behavior-item::Definitions::Fuel'[item_def])
          (reference_usage out reference 'fuelOut' : '3d-Function-based Behavior-item::Definitions::Fuel'[item_def])))
      (package 'Usages'
        (part_usage 'context'
          (part_usage composite 'storageTank' : '3d-Function-based Behavior-item::Definitions::StorageTank'[part_def])
          (flow_usage composite 'of')
          (part_usage composite 'pump' : '3d-Function-based Behavior-item::Definitions::Pump'[part_def]
            (perform_action_usage 'pumpFuel' : '3d-Function-based Behavior-item::Definitions::PumpFuel'[action_def]
              (reference_usage in reference 'fuelIn'
                (feature_value (=)))
              (reference_usage out reference 'fuelOut'
                (feature_value (=)))))
          (flow_usage composite 'of')
          (part_usage composite 'vehicle' : '3d-Function-based Behavior-item::Definitions::Vehicle'[part_def]
            (flow_usage composite 'fuelInPort')
            (part_usage composite 'fuelTank' : '3d-Function-based Behavior-item::Definitions::FuelTank'[part_def]
              (attribute_usage composite 'volumeMax' : 'Real'[unresolved])
              (attribute_usage composite 'fuelLevel' : 'Real'[unresolved]
                (feature_value (=)))
              (item_usage composite 'fuel' : '3d-Function-based Behavior-item::Definitions::Fuel'[item_def]
                (attribute_usage composite 'volume' : 'Real'[unresolved])))))))))
~~~
