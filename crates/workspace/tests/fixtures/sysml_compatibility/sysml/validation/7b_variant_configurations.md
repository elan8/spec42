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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "7b-Variant Configurations"))) (name "7b-Variant Configurations") (declared-name "7b-Variant Configurations")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "7b-Variant Configurations::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "7b-Variant Configurations::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "7b-Variant Configurations::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))) (name "DesignModel") (declared-name "DesignModel")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (name "Clutch") (declared-name "Clutch") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (name "ClutchPort") (declared-name "ClutchPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort::~ClutchPort"))) (name "~ClutchPort") (declared-name "~ClutchPort") (effective (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (name "Driveshaft") (declared-name "Driveshaft") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (name "Engine") (declared-name "Engine") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (name "FuelCmdPort") (declared-name "FuelCmdPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort::~FuelCmdPort"))) (name "~FuelCmdPort") (declared-name "~FuelCmdPort") (effective (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (name "RearAxleAssembly") (declared-name "RearAxleAssembly") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (name "ShaftPort_b") (declared-name "ShaftPort_b")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b::~ShaftPort_b"))) (name "~ShaftPort_b") (declared-name "~ShaftPort_b") (effective (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (name "ShaftPort_c") (declared-name "ShaftPort_c")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c::~ShaftPort_c"))) (name "~ShaftPort_c") (declared-name "~ShaftPort_c") (effective (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d"))) (name "ShaftPort_d") (declared-name "ShaftPort_d")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d::~ShaftPort_d"))) (name "~ShaftPort_d") (declared-name "~ShaftPort_d") (effective (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))) (name "VehicleToRoadPort") (declared-name "VehicleToRoadPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort::~VehicleToRoadPort"))) (name "~VehicleToRoadPort") (declared-name "~VehicleToRoadPort") (effective (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (name "WheelToRoadPort") (declared-name "WheelToRoadPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort::~WheelToRoadPort"))) (name "~WheelToRoadPort") (declared-name "~WheelToRoadPort") (effective (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (name "driveshaft") (declared-name "driveshaft") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (name "shaftPort_b") (declared-name "shaftPort_b") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft")))))
                    (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (name "shaftPort_c") (declared-name "shaftPort_c") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (name "rearWheels") (declared-name "rearWheels") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
                      )
                    )
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (name "clutch") (declared-name "clutch") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
                      )
                    )
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (declared (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel"))) (name "RequirementsModel") (declared-name "RequirementsModel")
          (contains
            (element (kind "requirement def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (name "EnginePerformanceRequirement") (declared-name "EnginePerformanceRequirement"))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (name "highPerformanceRequirement") (declared-name "highPerformanceRequirement"))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (name "normalPerformanceRequirement") (declared-name "normalPerformanceRequirement"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel"))) (name "VariabilityModel") (declared-name "VariabilityModel")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (name "anyVehicleConfig") (declared-name "anyVehicleConfig") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (name "engineChoice") (declared-name "engineChoice") (declared (properties (variation true) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))) (name "4cylEngine") (declared-name "4cylEngine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))) (name "6cylEngine") (declared-name "6cylEngine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (name "rearWheelChoice") (declared-name "rearWheelChoice") (declared (properties (variation true) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))) (name "narrowRimWheel") (declared-name "narrowRimWheel") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                        (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))) (name "wideRimWheel") (declared-name "wideRimWheel") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                      )
                    )
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (name "transmissionChoice") (declared-name "transmissionChoice") (declared (properties (variation true) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (name "automaticTransmission") (declared-name "automaticTransmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (name "clutch") (declared-name "clutch") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))))
                          (contains
                            (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch")))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (name "manualTransmission") (declared-name "manualTransmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (name "clutch") (declared-name "clutch") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))))
                          (contains
                            (element (kind "port") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch")))))
                          )
                        )
                      )
                    )
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (name "vehicleChoice") (declared-name "vehicleChoice") (declared (properties (variation true) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice::vehicle_c1"))) (name "vehicle_c1") (declared-name "vehicle_c1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice::vehicle_c2"))) (name "vehicle_c2") (declared-name "vehicle_c2") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (name "VariantDefinitions") (declared-name "VariantDefinitions")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (name "4CylEngine") (declared-name "4CylEngine") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (name "6CylEngine") (declared-name "6CylEngine") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (name "AutomaticClutch") (declared-name "AutomaticClutch") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (name "AutomaticClutchPort") (declared-name "AutomaticClutchPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort::~AutomaticClutchPort"))) (name "~AutomaticClutchPort") (declared-name "~AutomaticClutchPort") (effective (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (name "AutomaticTransmission") (declared-name "AutomaticTransmission") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (name "ManualClutch") (declared-name "ManualClutch") (declared))
            (element (kind "port def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (name "ManualClutchPort") (declared-name "ManualClutchPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort::~ManualClutchPort"))) (name "~ManualClutchPort") (declared-name "~ManualClutchPort") (effective (featuring-type (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (name "ManualTransmission") (declared-name "ManualTransmission") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (name "NarrowRimWheel") (declared-name "NarrowRimWheel") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (name "WideRimWheel") (declared-name "WideRimWheel") (declared))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "7b-Variant Configurations::forAll"))) (name "forAll") (declared-name "forAll"))
      )
    )
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
  )
  (relationships
    (bind (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (connect (source-expression "fuelCmdPort") (target-expression "engine::fuelCmdPort") (container-prefix "7b-Variant Configurations::DesignModel::vehicle")) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort::~ClutchPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort::~FuelCmdPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b::~ShaftPort_b"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c::~ShaftPort_c"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d::~ShaftPort_d"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort::~VehicleToRoadPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort::~WheelToRoadPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort::~AutomaticClutchPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort::~ManualClutchPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (to (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (satisfy (status pending-expression) (document "d0") (source-expression "engineRqtChoice") (target-expression "engineChoice") (container-prefix "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort::~ClutchPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort::~FuelCmdPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b::~ShaftPort_b"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c::~ShaftPort_c"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d::~ShaftPort_d"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort::~VehicleToRoadPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort::~WheelToRoadPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice::vehicle_c1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice::vehicle_c2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort::~AutomaticClutchPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort::~ManualClutchPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (status missing-prerequisite) (target "Parts::Part"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/7b_variant_configurations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 40))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 30 3) (end 30 20))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 35 4) (end 35 35))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 40 5) (end 40 34))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 45 4) (end 45 35))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 46 4) (end 46 35))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 51 5) (end 51 44))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 55 3) (end 55 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 86 3) (end 86 140))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 91 11) (end 91 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 100 3) (end 100 384))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 103 6) (end 103 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 108 6) (end 108 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 118 3) (end 118 558))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 119 4) (end 119 155))
      )
    )
  )
)
~~~
