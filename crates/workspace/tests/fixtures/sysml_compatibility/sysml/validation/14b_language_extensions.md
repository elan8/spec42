# META
~~~ini
description=SysML Validation (14-Language Extensions): 14b-Language Extensions
type=file
~~~
# SOURCE
~~~sysml
package '14b-Language-Extensions' {
	
	package LibraryModel {
		
		part def ECU;
		
	}
	
	package UserModel {
		
		package Definitions {
			private import LibraryModel::*;
			
			part def VehicleControlUnit :> ECU;
			part def EngineControlUnit :> ECU;
			
			part def Vehicle;
			part def Engine;
			part def CanBus;
			
			port def BusIF;
		}
		
		package Usages {
			private import Definitions::*;
			
			part vehicle1: Vehicle {
				part vehicleControlUnit : VehicleControlUnit {
					port busIF: ~BusIF;
				}
				
				connect vehicleControlUnit.busIF to canBus.vehicleControlIF;
				
				part canBus: CanBus {
					port vehicleControlIF: BusIF;
					port engineControlIF: BusIF;
					port sensorIF: BusIF;					
				}
				
				connect engine.engineControlUnit.busIF to canBus.engineControlIF;
				
				part engine: Engine {
					part engineControlUnit: EngineControlUnit {
						port busIF: ~BusIF;
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
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
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
  (package_def ''14b-Language-Extensions''
    (package_def 'LibraryModel'
      (part_def 'ECU'))
    (package_def 'UserModel'
      (package_def 'Definitions'
        (import_decl private 'LibraryModel::*')
        (part_def 'VehicleControlUnit' :> 'ECU')
        (part_def 'EngineControlUnit' :> 'ECU')
        (part_def 'Vehicle')
        (part_def 'Engine')
        (part_def 'CanBus')
        (port_def 'BusIF'))
      (package_def 'Usages'
        (import_decl private 'Definitions::*')
        (part_usage 'vehicle1' : 'Vehicle'
          (part_usage 'vehicleControlUnit' : 'VehicleControlUnit'
            (port_usage 'busIF' : ~'BusIF'))
          (connection_usage
            (connector_end)
            (connector_end))
          (part_usage 'canBus' : 'CanBus'
            (port_usage 'vehicleControlIF' : 'BusIF')
            (port_usage 'engineControlIF' : 'BusIF')
            (port_usage 'sensorIF' : 'BusIF'))
          (connection_usage
            (connector_end)
            (connector_end))
          (part_usage 'engine' : 'Engine'
            (part_usage 'engineControlUnit' : 'EngineControlUnit'
              (port_usage 'busIF' : ~'BusIF'))))))))
~~~
# FORMAT
~~~sysml
package '14b-Language-Extensions' {
    package LibraryModel {
        part def ECU;
    }

    package UserModel {
        package Definitions {
            private import LibraryModel::*;

            part def VehicleControlUnit :> ECU;
            part def EngineControlUnit :> ECU;

            part def Vehicle;
            part def Engine;
            part def CanBus;

            port def BusIF;
        }

        package Usages {
            private import Definitions::*;

            part vehicle1 : Vehicle {
                part vehicleControlUnit : VehicleControlUnit {
                    port busIF : ~BusIF;
                }

                connect vehicleControlUnit.busIF to canBus.vehicleControlIF;

                part canBus : CanBus {
                    port vehicleControlIF : BusIF;
                    port engineControlIF : BusIF;
                    port sensorIF : BusIF;
                }

                connect engine.engineControlUnit.busIF to canBus.engineControlIF;

                part engine : Engine {
                    part engineControlUnit : EngineControlUnit {
                        port busIF : ~BusIF;
                    }
                }
            }
        }
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package '14b-Language-Extensions'
      (package 'LibraryModel'
        (part_def 'ECU'))
      (package 'UserModel'
        (package 'Definitions'
          (namespace_import private -> '14b-Language-Extensions::LibraryModel'[package])
          (part_def 'VehicleControlUnit' :> '14b-Language-Extensions::LibraryModel::ECU'[part_def])
          (part_def 'EngineControlUnit' :> '14b-Language-Extensions::LibraryModel::ECU'[part_def])
          (part_def 'Vehicle')
          (part_def 'Engine')
          (part_def 'CanBus')
          (port_def 'BusIF'))
        (package 'Usages'
          (namespace_import private -> '14b-Language-Extensions::UserModel::Definitions'[package])
          (part_usage 'vehicle1' : '14b-Language-Extensions::UserModel::Definitions::Vehicle'[part_def]
            (part_usage composite 'vehicleControlUnit' : '14b-Language-Extensions::UserModel::Definitions::VehicleControlUnit'[part_def]
              (port_usage composite 'busIF' : '14b-Language-Extensions::UserModel::Definitions::BusIF'[port_def] ~ '14b-Language-Extensions::UserModel::Definitions::BusIF'[port_def]))
            (connection_usage composite
              (connector_end 'vehicleControlUnit.busIF')
              (connector_end 'canBus.vehicleControlIF'))
            (part_usage composite 'canBus' : '14b-Language-Extensions::UserModel::Definitions::CanBus'[part_def]
              (port_usage composite 'vehicleControlIF' : '14b-Language-Extensions::UserModel::Definitions::BusIF'[port_def])
              (port_usage composite 'engineControlIF' : '14b-Language-Extensions::UserModel::Definitions::BusIF'[port_def])
              (port_usage composite 'sensorIF' : '14b-Language-Extensions::UserModel::Definitions::BusIF'[port_def]))
            (connection_usage composite
              (connector_end 'engine.engineControlUnit.busIF')
              (connector_end 'canBus.engineControlIF'))
            (part_usage composite 'engine' : '14b-Language-Extensions::UserModel::Definitions::Engine'[part_def]
              (part_usage composite 'engineControlUnit' : '14b-Language-Extensions::UserModel::Definitions::EngineControlUnit'[part_def]
                (port_usage composite 'busIF' : '14b-Language-Extensions::UserModel::Definitions::BusIF'[port_def] ~ '14b-Language-Extensions::UserModel::Definitions::BusIF'[port_def])))))))))
~~~
