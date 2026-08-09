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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPerform,KwAction,UnrestrictedName,OpenCurly,
KwAction,UnrestrictedName,OpenCurly,
KwIn,KwItem,UnrestrictedName,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwOut,KwItem,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,OpenCurly,
KwPart,Ident,Colon,Ident,Eq,UnrestrictedName,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
KwFlow,UnrestrictedName,Dot,UnrestrictedName,
KwTo,UnrestrictedName,Dot,UnrestrictedName,Semicolon,
KwAction,UnrestrictedName,OpenCurly,
KwIn,KwItem,UnrestrictedName,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Eq,UnrestrictedName,OpenCurly,
KwPart,Ident,Colon,Ident,Eq,UnrestrictedName,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwBind,UnrestrictedName,Dot,UnrestrictedName,Dot,Ident,Eq,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPerform,KwAction,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''3e-Function-based Behavior-item''
    (import_decl public 'Definitions::*')
    (package_def 'Definitions'
      (item_def 'VehicleAssembly')
      (item_def 'AssembledVehicle' :> 'VehicleAssembly')
      (part_def 'Vehicle' :> 'AssembledVehicle')
      (part_def 'Transmission')
      (part_def 'Engine'))
    (package_def 'Usages'
      (part_usage 'AssemblyLine'
        (perform_action ''assemble vehicle''
          (action_usage ''assemble transmission into vehicle''
            (item_usage in ''vehicle assy without transmission or engine'' : 'VehicleAssembly')
            (item_usage in 'transmission' : 'Transmission'
              (comment))
            (item_usage out ''vehicle assy without engine'' : 'VehicleAssembly' value
              (part_usage 'transmission' : 'Transmission' value
                (comment))))
          (flow_usage ''assemble transmission into vehicle'')
          (action_usage ''assemble engine into vehicle''
            (item_usage in ''vehicle assy without engine'' : 'VehicleAssembly'
              (part_usage 'transmission' : 'Transmission'))
            (item_usage in 'engine' : 'Engine')
            (item_usage out 'assembledVehicle' : 'AssembledVehicle' value
              (part_usage 'engine' : 'Engine' value))))
        (binding_as_usage
          (connector_end)
          (connector_end))
        (part_usage 'vehicle' : 'Vehicle'
          (comment)
          (part_usage 'transmission' : 'Transmission')
          (part_usage 'engine' : 'Engine')
          (perform_action 'providePower'))))))
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

                flow 'assemble transmission into vehicle';

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

                part transmission : Transmission;
                part engine : Engine;

                perform action providePower;
            }
        }
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'assemble transmission into vehicle'
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'assemble transmission into vehicle'
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(model
  (namespace
    (package '3e-Function-based Behavior-item'
      (namespace_import public -> '3e-Function-based Behavior-item::Definitions'[package])
      (package 'Definitions'
        (item_def 'VehicleAssembly')
        (item_def 'AssembledVehicle' :> '3e-Function-based Behavior-item::Definitions::VehicleAssembly'[item_def])
        (part_def 'Vehicle' :> '3e-Function-based Behavior-item::Definitions::AssembledVehicle'[item_def])
        (part_def 'Transmission')
        (part_def 'Engine'))
      (package 'Usages'
        (part_usage 'AssemblyLine'
          (perform_action_usage 'assemble vehicle'
            (action_usage 'assemble transmission into vehicle'
              (item_usage in 'vehicle assy without transmission or engine' : '3e-Function-based Behavior-item::Definitions::VehicleAssembly'[item_def])
              (item_usage in 'transmission' : '3e-Function-based Behavior-item::Definitions::Transmission'[part_def])
              (item_usage out 'vehicle assy without engine' : '3e-Function-based Behavior-item::Definitions::VehicleAssembly'[item_def]
                (feature_value (=))
                (part_usage composite 'transmission' : '3e-Function-based Behavior-item::Definitions::Transmission'[part_def]
                  (feature_value (=)))))
            (flow_usage 'assemble transmission into vehicle')
            (action_usage 'assemble engine into vehicle'
              (item_usage in 'vehicle assy without engine' : '3e-Function-based Behavior-item::Definitions::VehicleAssembly'[item_def]
                (part_usage composite 'transmission' : '3e-Function-based Behavior-item::Definitions::Transmission'[part_def]))
              (item_usage in 'engine' : '3e-Function-based Behavior-item::Definitions::Engine'[part_def])
              (item_usage out 'assembledVehicle' : '3e-Function-based Behavior-item::Definitions::AssembledVehicle'[item_def]
                (feature_value (=))
                (part_usage composite 'engine' : '3e-Function-based Behavior-item::Definitions::Engine'[part_def]
                  (feature_value (=))))))
          (binding_connector_def
            (connector_end ''assemble vehicle'.'assemble engine into vehicle'.assembledVehicle')
            (connector_end 'vehicle'))
          (part_usage composite 'vehicle' : '3e-Function-based Behavior-item::Definitions::Vehicle'[part_def]
            (part_usage composite 'transmission' : '3e-Function-based Behavior-item::Definitions::Transmission'[part_def])
            (part_usage composite 'engine' : '3e-Function-based Behavior-item::Definitions::Engine'[part_def])
            (perform_action_usage 'providePower')))))))
~~~
