# META
~~~ini
description=SysML Validation (07-Variant Configuration): 7b-Variant Configurations
type=file
~~~
# SOURCE
~~~sysml
package '7b-Variant Configurations' {
	private import RequirementsModel::*;
	private import DesignModel::*;
	private import VariantDefinitions::*;
	private import ControlFunctions::forAll;
	
	package RequirementsModel {
		requirement def EnginePerformanceRequirement;
		requirement highPerformanceRequirement : EnginePerformanceRequirement;
		requirement normalPerformanceRequirement : EnginePerformanceRequirement;
	}
	
	package DesignModel {
		part def Vehicle;
		part def Engine;
		part def Transmission;
		part def Clutch;
		part def Driveshaft;
		part def RearAxleAssembly;
		part def Wheel;
		
		port def FuelCmdPort;
		port def ClutchPort;
		port def ShaftPort_b;
		port def ShaftPort_c;
		port def ShaftPort_d;
		port def VehicleToRoadPort;
		port def WheelToRoadPort;
		
		part vehicle : Vehicle {
			port fuelCmdPort;
			
			bind fuelCmdPort = engine.fuelCmdPort;
			
			part engine : Engine[1] {
				port fuelCmdPort : FuelCmdPort;
			}
			
			part transmission : Transmission[1] {
				part clutch: Clutch[1] {
					port clutchPort : ClutchPort;
				}
			}
			
			part driveshaft : Driveshaft[1] {
				port shaftPort_b : ShaftPort_b;
				port shaftPort_c : ShaftPort_c;
			}
			
			part rearAxleAssembly : RearAxleAssembly {
				part rearWheels : Wheel[2] {
					port wheelToRoadPort : WheelToRoadPort;
				}
			}
			
			port vehicleToRoadPort : VehicleToRoadPort {
				port wheelToRoadPort : WheelToRoadPort[2];
			}
		}
	}
	
	package VariantDefinitions {
		part def '4CylEngine' :> Engine;
		part def '6CylEngine' :> Engine;
		
		part def ManualTransmission :> Transmission;
		part def AutomaticTransmission :> Transmission;
		
		part def ManualClutch :> Clutch;
		part def AutomaticClutch :> Clutch;
		
		port def ManualClutchPort :> ClutchPort;
		port def AutomaticClutchPort :> ClutchPort;
		
		part def NarrowRimWheel :> Wheel;
		part def WideRimWheel :> Wheel;		
	}
	
	package VariabilityModel {
		part anyVehicleConfig :> vehicle {
			
			variation requirement engineRqtChoice : EnginePerformanceRequirement {
				variant highPerformanceRequirement;
				variant normalPerformanceRequirement;
			}
			
			variation part engineChoice :>> engine {
				variant part '4cylEngine' : '4CylEngine';
				variant part '6cylEngine' : '6CylEngine';
			}
			
			satisfy engineRqtChoice by engineChoice;
			
			assert constraint 'engine choice constraint' {
				if engineRqtChoice == engineRqtChoice::highPerformanceRequirement? 
					engineChoice == engineChoice::'6cylEngine' 
				else
					engineChoice == engineChoice::'4cylEngine'
			}
			
			variation part transmissionChoice :>> transmission {
				variant part manualTransmission : ManualTransmission {
					part :>> clutch : ManualClutch {
						port :>> clutchPort : ManualClutchPort;
					}
				}
				variant part automaticTransmission : AutomaticTransmission {
					part :>> clutch : AutomaticClutch {
						port :>> clutchPort : AutomaticClutchPort;
					}
				}
			}
			
			assert constraint 'engine-transmission selection constraint' {
				(engineChoice == engineChoice::'4cylEngine' and transmissionChoice == transmissionChoice::manualTransmission) xor
				(engineChoice == engineChoice::'6cylEngine' and transmissionChoice == transmissionChoice::automaticTransmission)
			}
			
			part :>> rearAxleAssembly {
				variation part rearWheelChoice :>> rearWheels {
					variant part narrowRimWheel : NarrowRimWheel;
					variant part wideRimWheel : WideRimWheel;
				}
			
    			assert constraint 'engine-wheel selection constraint' {
    				(engineChoice == engineChoice::'4cylEngine' and 
    					rearWheelChoice->forAll {in ref w; w == rearWheelChoice::narrowRimWheel}) xor
    				(engineChoice == engineChoice::'6cylEngine' and 
    					rearWheelChoice->forAll {in ref w; w == rearWheelChoice::wideRimWheel})
    			}
            }
			
		}
		
		variation part vehicleChoice :> anyVehicleConfig {
			variant part vehicle_c1;
			variant part vehicle_c2;
		}	
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPackage,Ident,OpenCurly,
KwRequirement,KwDef,Ident,Semicolon,
KwRequirement,Ident,Colon,Ident,Semicolon,
KwRequirement,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Semicolon,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPort,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPort,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPort,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwVariation,KwRequirement,Ident,Colon,Ident,OpenCurly,
KwVariant,Ident,Semicolon,
KwVariant,Ident,Semicolon,
CloseCurly,
KwVariation,KwPart,Ident,ColonGtGt,Ident,OpenCurly,
KwVariant,KwPart,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwVariant,KwPart,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
CloseCurly,
KwSatisfy,Ident,KwBy,Ident,Semicolon,
KwAssert,KwConstraint,UnrestrictedName,OpenCurly,
KwIf,Ident,EqEq,Ident,ColonColon,Ident,Question,
Ident,EqEq,Ident,ColonColon,UnrestrictedName,
KwElse,
Ident,EqEq,Ident,ColonColon,UnrestrictedName,
CloseCurly,
KwVariation,KwPart,Ident,ColonGtGt,Ident,OpenCurly,
KwVariant,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwVariant,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwAssert,KwConstraint,UnrestrictedName,OpenCurly,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,Ident,EqEq,Ident,ColonColon,Ident,CloseParen,KwXor,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,Ident,EqEq,Ident,ColonColon,Ident,CloseParen,
CloseCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwVariation,KwPart,Ident,ColonGtGt,Ident,OpenCurly,
KwVariant,KwPart,Ident,Colon,Ident,Semicolon,
KwVariant,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAssert,KwConstraint,UnrestrictedName,OpenCurly,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,
Ident,Arrow,Ident,OpenCurly,KwIn,KwRef,Ident,Semicolon,Ident,EqEq,Ident,ColonColon,Ident,CloseCurly,CloseParen,KwXor,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,
Ident,Arrow,Ident,OpenCurly,KwIn,KwRef,Ident,Semicolon,Ident,EqEq,Ident,ColonColon,Ident,CloseCurly,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,
KwVariation,KwPart,Ident,ColonGt,Ident,OpenCurly,
KwVariant,KwPart,Ident,Semicolon,
KwVariant,KwPart,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''7b-Variant Configurations''
    (import_decl private 'RequirementsModel::*')
    (import_decl private 'DesignModel::*')
    (import_decl private 'VariantDefinitions::*')
    (import_decl private 'ControlFunctions::forAll')
    (package_def 'RequirementsModel'
      (requirement_def 'EnginePerformanceRequirement')
      (requirement_usage 'highPerformanceRequirement' : 'EnginePerformanceRequirement')
      (requirement_usage 'normalPerformanceRequirement' : 'EnginePerformanceRequirement'))
    (package_def 'DesignModel'
      (part_def 'Vehicle')
      (part_def 'Engine')
      (part_def 'Transmission')
      (part_def 'Clutch')
      (part_def 'Driveshaft')
      (part_def 'RearAxleAssembly')
      (part_def 'Wheel')
      (port_def 'FuelCmdPort')
      (port_def 'ClutchPort')
      (port_def 'ShaftPort_b')
      (port_def 'ShaftPort_c')
      (port_def 'ShaftPort_d')
      (port_def 'VehicleToRoadPort')
      (port_def 'WheelToRoadPort')
      (part_usage 'vehicle' : 'Vehicle'
        (port_usage 'fuelCmdPort')
        (binding_as_usage
          (connector_end)
          (connector_end))
        (part_usage 'engine' : 'Engine' multiplicity
          (port_usage 'fuelCmdPort' : 'FuelCmdPort'))
        (part_usage 'transmission' : 'Transmission' multiplicity
          (part_usage 'clutch' : 'Clutch' multiplicity
            (port_usage 'clutchPort' : 'ClutchPort')))
        (part_usage 'driveshaft' : 'Driveshaft' multiplicity
          (port_usage 'shaftPort_b' : 'ShaftPort_b')
          (port_usage 'shaftPort_c' : 'ShaftPort_c'))
        (part_usage 'rearAxleAssembly' : 'RearAxleAssembly'
          (part_usage 'rearWheels' : 'Wheel' multiplicity
            (port_usage 'wheelToRoadPort' : 'WheelToRoadPort')))
        (port_usage 'vehicleToRoadPort' : 'VehicleToRoadPort'
          (port_usage 'wheelToRoadPort' : 'WheelToRoadPort' multiplicity))))
    (package_def 'VariantDefinitions'
      (part_def ''4CylEngine'' :> 'Engine')
      (part_def ''6CylEngine'' :> 'Engine')
      (part_def 'ManualTransmission' :> 'Transmission')
      (part_def 'AutomaticTransmission' :> 'Transmission')
      (part_def 'ManualClutch' :> 'Clutch')
      (part_def 'AutomaticClutch' :> 'Clutch')
      (port_def 'ManualClutchPort' :> 'ClutchPort')
      (port_def 'AutomaticClutchPort' :> 'ClutchPort')
      (part_def 'NarrowRimWheel' :> 'Wheel')
      (part_def 'WideRimWheel' :> 'Wheel'))
    (package_def 'VariabilityModel'
      (part_usage 'anyVehicleConfig' :> 'vehicle'
        (requirement_usage variation 'engineRqtChoice' : 'EnginePerformanceRequirement'
          (variant_usage
            (default_ref_usage 'highPerformanceRequirement'))
          (variant_usage
            (default_ref_usage 'normalPerformanceRequirement')))
        (part_usage variation 'engineChoice' :>> 'engine'
          (variant_usage
            (part_usage ''4cylEngine'' : ''4CylEngine''))
          (variant_usage
            (part_usage ''6cylEngine'' : ''6CylEngine'')))
        (sysml_decl 'engineRqtChoice')
        (sysml_decl ''engine choice constraint''
          (result_expr_member))
        (part_usage variation 'transmissionChoice' :>> 'transmission'
          (variant_usage
            (part_usage 'manualTransmission' : 'ManualTransmission'
              (part_usage :>> 'clutch' : 'ManualClutch'
                (port_usage :>> 'clutchPort' : 'ManualClutchPort'))))
          (variant_usage
            (part_usage 'automaticTransmission' : 'AutomaticTransmission'
              (part_usage :>> 'clutch' : 'AutomaticClutch'
                (port_usage :>> 'clutchPort' : 'AutomaticClutchPort')))))
        (sysml_decl ''engine-transmission selection constraint''
          (result_expr_member))
        (part_usage :>> 'rearAxleAssembly'
          (part_usage variation 'rearWheelChoice' :>> 'rearWheels'
            (variant_usage
              (part_usage 'narrowRimWheel' : 'NarrowRimWheel'))
            (variant_usage
              (part_usage 'wideRimWheel' : 'WideRimWheel')))
          (sysml_decl ''engine-wheel selection constraint''
            (result_expr_member))))
      (part_usage variation 'vehicleChoice' :> 'anyVehicleConfig'
        (variant_usage
          (part_usage 'vehicle_c1'))
        (variant_usage
          (part_usage 'vehicle_c2'))))))
~~~
# FORMAT
~~~sysml
package '7b-Variant Configurations' {
    private import RequirementsModel::*;
    private import DesignModel::*;
    private import VariantDefinitions::*;
    private import ControlFunctions::forAll;

    package RequirementsModel {
        requirement def EnginePerformanceRequirement;
        requirement highPerformanceRequirement : EnginePerformanceRequirement;
        requirement normalPerformanceRequirement : EnginePerformanceRequirement;
    }

    package DesignModel {
        part def Vehicle;
        part def Engine;
        part def Transmission;
        part def Clutch;
        part def Driveshaft;
        part def RearAxleAssembly;
        part def Wheel;

        port def FuelCmdPort;
        port def ClutchPort;
        port def ShaftPort_b;
        port def ShaftPort_c;
        port def ShaftPort_d;
        port def VehicleToRoadPort;
        port def WheelToRoadPort;

        part vehicle : Vehicle {
            port fuelCmdPort;

            bind fuelCmdPort = engine.fuelCmdPort;

            part engine : Engine [1] {
                port fuelCmdPort : FuelCmdPort;
            }

            part transmission : Transmission [1] {
                part clutch : Clutch [1] {
                    port clutchPort : ClutchPort;
                }
            }

            part driveshaft : Driveshaft [1] {
                port shaftPort_b : ShaftPort_b;
                port shaftPort_c : ShaftPort_c;
            }

            part rearAxleAssembly : RearAxleAssembly {
                part rearWheels : Wheel [2] {
                    port wheelToRoadPort : WheelToRoadPort;
                }
            }

            port vehicleToRoadPort : VehicleToRoadPort {
                port wheelToRoadPort : WheelToRoadPort [2];
            }
        }
    }

    package VariantDefinitions {
        part def '4CylEngine' :> Engine;
        part def '6CylEngine' :> Engine;

        part def ManualTransmission :> Transmission;
        part def AutomaticTransmission :> Transmission;

        part def ManualClutch :> Clutch;
        part def AutomaticClutch :> Clutch;

        port def ManualClutchPort :> ClutchPort;
        port def AutomaticClutchPort :> ClutchPort;

        part def NarrowRimWheel :> Wheel;
        part def WideRimWheel :> Wheel;
    }

    package VariabilityModel {
        part anyVehicleConfig :> vehicle {
            variation requirement engineRqtChoice : EnginePerformanceRequirement {
                variant highPerformanceRequirement;
                variant normalPerformanceRequirement;
            }

            variation part engineChoice :>> engine {
                variant part '4cylEngine' : '4CylEngine';
                variant part '6cylEngine' : '6CylEngine';
            }

            satisfy engineRqtChoice by engineChoice;

            assert constraint 'engine choice constraint' {
                = if engineRqtChoice == engineRqtChoice::highPerformanceRequirement ? engineChoice == engineChoice::'6cylEngine' else engineChoice == engineChoice::'4cylEngine';
            }

            variation part transmissionChoice :>> transmission {
                variant part manualTransmission : ManualTransmission {
					part :>> clutch : ManualClutch {
						port :>> clutchPort : ManualClutchPort;
					}
				}
                variant part automaticTransmission : AutomaticTransmission {
					part :>> clutch : AutomaticClutch {
						port :>> clutchPort : AutomaticClutchPort;
					}
				}
            }

            assert constraint 'engine-transmission selection constraint' {
                = (engineChoice == engineChoice::'4cylEngine' and transmissionChoice == transmissionChoice::manualTransmission) xor (engineChoice == engineChoice::'6cylEngine' and transmissionChoice == transmissionChoice::automaticTransmission);
            }

            part :>> rearAxleAssembly {
                variation part rearWheelChoice :>> rearWheels {
                    variant part narrowRimWheel : NarrowRimWheel;
                    variant part wideRimWheel : WideRimWheel;
                }

                assert constraint 'engine-wheel selection constraint' {
                    = (engineChoice == engineChoice::'4cylEngine' and rearWheelChoice->forAll {in ref w; w == rearWheelChoice::narrowRimWheel}) xor (engineChoice == engineChoice::'6cylEngine' and rearWheelChoice->forAll {in ref w; w == rearWheelChoice::wideRimWheel});
                }
            }
        }

        variation part vehicleChoice :> anyVehicleConfig {
            variant part vehicle_c1;
            variant part vehicle_c2;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'engineRqtChoice'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'engineRqtChoice'
~~~
# SMG
~~~
(model
  (namespace
    (package '7b-Variant Configurations'
      (namespace_import private -> '7b-Variant Configurations::RequirementsModel'[package])
      (namespace_import private -> '7b-Variant Configurations::DesignModel'[package])
      (namespace_import private -> '7b-Variant Configurations::VariantDefinitions'[package])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (package 'RequirementsModel'
        (requirement_def 'EnginePerformanceRequirement')
        (requirement_usage 'highPerformanceRequirement' : '7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement'[requirement_def])
        (requirement_usage 'normalPerformanceRequirement' : '7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement'[requirement_def]))
      (package 'DesignModel'
        (part_def 'Vehicle')
        (part_def 'Engine')
        (part_def 'Transmission')
        (part_def 'Clutch')
        (part_def 'Driveshaft')
        (part_def 'RearAxleAssembly')
        (part_def 'Wheel')
        (port_def 'FuelCmdPort')
        (port_def 'ClutchPort')
        (port_def 'ShaftPort_b')
        (port_def 'ShaftPort_c')
        (port_def 'ShaftPort_d')
        (port_def 'VehicleToRoadPort')
        (port_def 'WheelToRoadPort')
        (part_usage 'vehicle' : '7b-Variant Configurations::DesignModel::Vehicle'[part_def]
          (port_usage composite 'fuelCmdPort')
          (binding_connector_def
            (connector_end 'fuelCmdPort')
            (connector_end 'engine.fuelCmdPort'))
          (part_usage composite 'engine' : '7b-Variant Configurations::DesignModel::Engine'[part_def]
            (multiplicity_range [1])
            (port_usage composite 'fuelCmdPort' : '7b-Variant Configurations::DesignModel::FuelCmdPort'[port_def]))
          (part_usage composite 'transmission' : '7b-Variant Configurations::DesignModel::Transmission'[part_def]
            (multiplicity_range [1])
            (part_usage composite 'clutch' : '7b-Variant Configurations::DesignModel::Clutch'[part_def]
              (multiplicity_range [1])
              (port_usage composite 'clutchPort' : '7b-Variant Configurations::DesignModel::ClutchPort'[port_def])))
          (part_usage composite 'driveshaft' : '7b-Variant Configurations::DesignModel::Driveshaft'[part_def]
            (multiplicity_range [1])
            (port_usage composite 'shaftPort_b' : '7b-Variant Configurations::DesignModel::ShaftPort_b'[port_def])
            (port_usage composite 'shaftPort_c' : '7b-Variant Configurations::DesignModel::ShaftPort_c'[port_def]))
          (part_usage composite 'rearAxleAssembly' : '7b-Variant Configurations::DesignModel::RearAxleAssembly'[part_def]
            (part_usage composite 'rearWheels' : '7b-Variant Configurations::DesignModel::Wheel'[part_def]
              (multiplicity_range [2])
              (port_usage composite 'wheelToRoadPort' : '7b-Variant Configurations::DesignModel::WheelToRoadPort'[port_def])))
          (port_usage composite 'vehicleToRoadPort' : '7b-Variant Configurations::DesignModel::VehicleToRoadPort'[port_def]
            (port_usage composite 'wheelToRoadPort' : '7b-Variant Configurations::DesignModel::WheelToRoadPort'[port_def]
              (multiplicity_range [2])))))
      (package 'VariantDefinitions'
        (part_def '4CylEngine' :> '7b-Variant Configurations::DesignModel::Engine'[part_def])
        (part_def '6CylEngine' :> '7b-Variant Configurations::DesignModel::Engine'[part_def])
        (part_def 'ManualTransmission' :> '7b-Variant Configurations::DesignModel::Transmission'[part_def])
        (part_def 'AutomaticTransmission' :> '7b-Variant Configurations::DesignModel::Transmission'[part_def])
        (part_def 'ManualClutch' :> '7b-Variant Configurations::DesignModel::Clutch'[part_def])
        (part_def 'AutomaticClutch' :> '7b-Variant Configurations::DesignModel::Clutch'[part_def])
        (port_def 'ManualClutchPort' :> '7b-Variant Configurations::DesignModel::ClutchPort'[port_def])
        (port_def 'AutomaticClutchPort' :> '7b-Variant Configurations::DesignModel::ClutchPort'[port_def])
        (part_def 'NarrowRimWheel' :> '7b-Variant Configurations::DesignModel::Wheel'[part_def])
        (part_def 'WideRimWheel' :> '7b-Variant Configurations::DesignModel::Wheel'[part_def]))
      (package 'VariabilityModel'
        (part_usage 'anyVehicleConfig' :> '7b-Variant Configurations::DesignModel::vehicle'[part_usage]
          (requirement_usage variation composite 'engineRqtChoice' : '7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement'[requirement_def]
            (variant_usage
              (reference_usage reference 'highPerformanceRequirement'))
            (variant_usage
              (reference_usage reference 'normalPerformanceRequirement')))
          (part_usage variation composite 'engineChoice' :>> '7b-Variant Configurations::DesignModel::vehicle::engine'[part_usage]
            (variant_usage
              (part_usage composite '4cylEngine' : '7b-Variant Configurations::VariantDefinitions::4CylEngine'[part_def]))
            (variant_usage
              (part_usage composite '6cylEngine' : '7b-Variant Configurations::VariantDefinitions::6CylEngine'[part_def])))
          (satisfy_requirement_usage 'engineRqtChoice' by '7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice'[part_usage])
          (assert_constraint_usage 'engine choice constraint'
            (result_expr_membership))
          (part_usage variation composite 'transmissionChoice' :>> '7b-Variant Configurations::DesignModel::vehicle::transmission'[part_usage]
            (variant_usage
              (part_usage composite 'manualTransmission' : '7b-Variant Configurations::VariantDefinitions::ManualTransmission'[part_def]
                (part_usage composite :>> '7b-Variant Configurations::DesignModel::vehicle::transmission::clutch'[part_usage] : '7b-Variant Configurations::VariantDefinitions::ManualClutch'[part_def]
                  (port_usage composite :>> '7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort'[port_usage] : '7b-Variant Configurations::VariantDefinitions::ManualClutchPort'[port_def]))))
            (variant_usage
              (part_usage composite 'automaticTransmission' : '7b-Variant Configurations::VariantDefinitions::AutomaticTransmission'[part_def]
                (part_usage composite :>> '7b-Variant Configurations::DesignModel::vehicle::transmission::clutch'[part_usage] : '7b-Variant Configurations::VariantDefinitions::AutomaticClutch'[part_def]
                  (port_usage composite :>> '7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort'[port_usage] : '7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort'[port_def])))))
          (assert_constraint_usage 'engine-transmission selection constraint'
            (result_expr_membership))
          (part_usage composite :>> '7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly'[part_usage]
            (part_usage variation composite 'rearWheelChoice' :>> '7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels'[part_usage]
              (variant_usage
                (part_usage composite 'narrowRimWheel' : '7b-Variant Configurations::VariantDefinitions::NarrowRimWheel'[part_def]))
              (variant_usage
                (part_usage composite 'wideRimWheel' : '7b-Variant Configurations::VariantDefinitions::WideRimWheel'[part_def])))
            (assert_constraint_usage 'engine-wheel selection constraint'
              (result_expr_membership))))
        (part_usage variation 'vehicleChoice' :> '7b-Variant Configurations::VariabilityModel::anyVehicleConfig'[part_usage]
          (variant_usage
            (part_usage composite 'vehicle_c1'))
          (variant_usage
            (part_usage composite 'vehicle_c2')))))))
~~~
