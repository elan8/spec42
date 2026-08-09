# META
~~~ini
description=SysML Example (Vehicle): VehicleUsages
type=file
~~~
# SOURCE
~~~sysml
package VehicleUsages {
	doc
	/*
	 * Example usages of elements from the vehicle definitions model.
	 */

	private import SI::N;
	private import SI::m;
	private import ScalarFunctions::*;

	public import VehicleDefinitions::*;

	/* VALUES */	 
	T1 = 10.0 [N * m];
	T2 = 20.0 [N * m];
	
	/* PARTS */	
	part narrowRimWheel: Wheel {
		doc /* Narrow-rim wheel configuration with 4 to 5 lugbolts. */

		part lugbolt: Lugbolt[4..5];
	}
	
	part wideRimWheel: Wheel {
		doc /* Wide-rim wheel configuration with 4 to 6 lugbolts. */	

		part lugbolt: Lugbolt[4..6];
	}

	part vehicle_C1: Vehicle {
		doc /* Basic Vehicle configuration showing a part hierarchy. */

		part frontAxleAssembly: AxleAssembly {
			part frontWheel[2] subsets narrowRimWheel {
				part redefines lugbolt[4] {
					attribute redefines tighteningTorque = T1;
				}
			}
			part frontAxle: Axle;
		}		
		part rearAxleAssembly: AxleAssembly {
			part rearWheel[2] subsets wideRimWheel {
				part redefines lugbolt[6] {
					attribute redefines tighteningTorque = T2;
				}
			}
			part rearAxle: Axle;			
		}
	}
	
	part vehicle_C2 subsets vehicle_C1 {
		doc /* Specialized configuration with part-specific ports. */

		part redefines frontAxleAssembly {
			part leftFrontWheel subsets frontWheel = frontWheel#(1);
			part rightFrontWheel subsets frontWheel = frontWheel#(2);
			
			interface leftFrontMount: Mounting connect 
				frontAxle.leftMountingPoint to leftFrontWheel.hub;
				
			interface rightFrontMount: Mounting connect 
				frontAxle.rightMountingPoint to rightFrontWheel.hub;
		}
		
		part rearAxleAssembly redefines vehicle_C1::rearAxleAssembly {
			part leftRearWheel subsets rearWheel = rearWheel#(1);
			part rightRearWheel subsets rearWheel = rearWheel#(2);

			interface leftRearMount: Mounting connect 
				rearAxle.leftMountingPoint to leftRearWheel.hub;
				
			interface rightRearMount: Mounting connect 
				rearAxle.rightMountingPoint to rightRearWheel.hub;
		}		
	}
	
	part vehicle_C3 subsets vehicle_C2 {
		doc /* Further specialized configuration with a connection to a deeply-nested port. */

		
		part transmission: Transmission {
			port drive: ~DriveIF;
		}
		
		part redefines rearAxleAssembly {
			part redefines rearAxle {
				port drive: DriveIF;
			}
		}
		
		interface driveShaft connect 
			transDrive ::> transmission.drive to axleDrive ::> rearAxleAssembly.rearAxle.drive {
			flow transDrive.driveTorque to axleDrive.driveTorque;
		}		
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Star,Ident,CloseSquare,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,Star,Ident,CloseSquare,Semicolon,
RegularComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,KwSubsets,Ident,OpenCurly,
KwDoc,RegularComment,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwInterface,Ident,Colon,Ident,KwConnect,
Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwInterface,Ident,Colon,Ident,KwConnect,
Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwPart,Ident,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwInterface,Ident,Colon,Ident,KwConnect,
Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwInterface,Ident,Colon,Ident,KwConnect,
Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,KwSubsets,Ident,OpenCurly,
KwDoc,RegularComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwInterface,Ident,KwConnect,
Ident,ColonColonGt,Ident,Dot,Ident,KwTo,Ident,ColonColonGt,Ident,Dot,Ident,Dot,Ident,OpenCurly,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleUsages'
    (documentation)
    (import_decl private 'SI::N')
    (import_decl private 'SI::m')
    (import_decl private 'ScalarFunctions::*')
    (import_decl public 'VehicleDefinitions::*')
    (comment)
    (feature_def 'T1' value)
    (feature_def 'T2' value)
    (comment)
    (part_usage 'narrowRimWheel' : 'Wheel'
      (documentation)
      (part_usage 'lugbolt' : 'Lugbolt' multiplicity))
    (part_usage 'wideRimWheel' : 'Wheel'
      (documentation)
      (part_usage 'lugbolt' : 'Lugbolt' multiplicity))
    (part_usage 'vehicle_C1' : 'Vehicle'
      (documentation)
      (part_usage 'frontAxleAssembly' : 'AxleAssembly'
        (part_usage 'frontWheel' :> 'narrowRimWheel' multiplicity
          (part_usage :>> 'lugbolt' multiplicity
            (attribute_usage :>> 'tighteningTorque' value)))
        (part_usage 'frontAxle' : 'Axle'))
      (part_usage 'rearAxleAssembly' : 'AxleAssembly'
        (part_usage 'rearWheel' :> 'wideRimWheel' multiplicity
          (part_usage :>> 'lugbolt' multiplicity
            (attribute_usage :>> 'tighteningTorque' value)))
        (part_usage 'rearAxle' : 'Axle')))
    (part_usage 'vehicle_C2' :> 'vehicle_C1'
      (documentation)
      (part_usage :>> 'frontAxleAssembly'
        (part_usage 'leftFrontWheel' :> 'frontWheel' value)
        (part_usage 'rightFrontWheel' :> 'frontWheel' value)
        (interface_usage 'Mounting' 'leftFrontMount'
          (connector_end)
          (connector_end))
        (interface_usage 'Mounting' 'rightFrontMount'
          (connector_end)
          (connector_end)))
      (part_usage 'rearAxleAssembly' :>> 'vehicle_C1::rearAxleAssembly'
        (part_usage 'leftRearWheel' :> 'rearWheel' value)
        (part_usage 'rightRearWheel' :> 'rearWheel' value)
        (interface_usage 'Mounting' 'leftRearMount'
          (connector_end)
          (connector_end))
        (interface_usage 'Mounting' 'rightRearMount'
          (connector_end)
          (connector_end))))
    (part_usage 'vehicle_C3' :> 'vehicle_C2'
      (documentation)
      (part_usage 'transmission' : 'Transmission'
        (port_usage 'drive' : ~'DriveIF'))
      (part_usage :>> 'rearAxleAssembly'
        (part_usage :>> 'rearAxle'
          (port_usage 'drive' : 'DriveIF')))
      (interface_usage 'driveShaft'
        (connector_end)
        (connector_end)
        (flow_usage 'transDrive')))))
~~~
# FORMAT
~~~sysml
package VehicleUsages {
    doc /*
	 * Example usages of elements from the vehicle definitions model.
	 */

    private import SI::N;
    private import SI::m;
    private import ScalarFunctions::*;

    public import VehicleDefinitions::*;

    /* VALUES */
    T1 = 10.0 [N * m];
    T2 = 20.0 [N * m];

    /* PARTS */
    part narrowRimWheel : Wheel {
        doc /* Narrow-rim wheel configuration with 4 to 5 lugbolts. */

        part lugbolt : Lugbolt [4..5];
    }

    part wideRimWheel : Wheel {
        doc /* Wide-rim wheel configuration with 4 to 6 lugbolts. */

        part lugbolt : Lugbolt [4..6];
    }

    part vehicle_C1 : Vehicle {
        doc /* Basic Vehicle configuration showing a part hierarchy. */

        part frontAxleAssembly : AxleAssembly {
            part frontWheel subsets narrowRimWheel [2] {
                part redefines lugbolt [4] {
                    attribute redefines tighteningTorque = T1;
                }
            }
            part frontAxle : Axle;
        }
        part rearAxleAssembly : AxleAssembly {
            part rearWheel subsets wideRimWheel [2] {
                part redefines lugbolt [6] {
                    attribute redefines tighteningTorque = T2;
                }
            }
            part rearAxle : Axle;
        }
    }

    part vehicle_C2 subsets vehicle_C1 {
        doc /* Specialized configuration with part-specific ports. */

        part redefines frontAxleAssembly {
            part leftFrontWheel subsets frontWheel = frontWheel#(1);
            part rightFrontWheel subsets frontWheel = frontWheel#(2);

            interface leftFrontMount : Mounting connect frontAxle.leftMountingPoint to leftFrontWheel.hub;

            interface rightFrontMount : Mounting connect frontAxle.rightMountingPoint to rightFrontWheel.hub;
        }

        part rearAxleAssembly redefines vehicle_C1::rearAxleAssembly {
            part leftRearWheel subsets rearWheel = rearWheel#(1);
            part rightRearWheel subsets rearWheel = rearWheel#(2);

            interface leftRearMount : Mounting connect rearAxle.leftMountingPoint to leftRearWheel.hub;

            interface rightRearMount : Mounting connect rearAxle.rightMountingPoint to rightRearWheel.hub;
        }
    }

    part vehicle_C3 subsets vehicle_C2 {
        doc /* Further specialized configuration with a connection to a deeply-nested port. */

        part transmission : Transmission {
            port drive : ~DriveIF;
        }

        part redefines rearAxleAssembly {
            part redefines rearAxle {
                port drive : DriveIF;
            }
        }

        interface driveShaft connect transDrive ::> transmission.drive to axleDrive ::> rearAxleAssembly.rearAxle.drive {
            flow transDrive;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.redefinition_featuring_type_overlap
semantic.invalid_connection_end_count
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Lugbolt'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Lugbolt'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'AxleAssembly'
semantic.unresolved_name 'tighteningTorque'
semantic.unresolved_name 'Axle'
semantic.unresolved_name 'AxleAssembly'
semantic.unresolved_name 'tighteningTorque'
semantic.unresolved_name 'Axle'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Transmission'
semantic.unresolved_name 'DriveIF'
semantic.unresolved_name 'DriveIF'
semantic.unresolved_name 'DriveIF'
~~~
# PROBLEMS
~~~
semantic.redefinition_featuring_type_overlap
semantic.invalid_connection_end_count
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Lugbolt'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Lugbolt'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'AxleAssembly'
semantic.unresolved_name 'tighteningTorque'
semantic.unresolved_name 'Axle'
semantic.unresolved_name 'AxleAssembly'
semantic.unresolved_name 'tighteningTorque'
semantic.unresolved_name 'Axle'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Transmission'
semantic.unresolved_name 'DriveIF'
semantic.unresolved_name 'DriveIF'
semantic.unresolved_name 'DriveIF'
~~~
# SMG
~~~
(model
  (namespace
    (package 'VehicleUsages'
      (documentation)
      (membership_import private -> 'SI::N'[unresolved])
      (membership_import private -> 'SI::m'[unresolved])
      (namespace_import private -> 'ScalarFunctions'[unresolved])
      (namespace_import public -> 'VehicleDefinitions'[unresolved])
      (feature_def 'T1'
        (feature_value (=)))
      (feature_def 'T2'
        (feature_value (=)))
      (part_usage 'narrowRimWheel' : 'Wheel'[unresolved]
        (documentation)
        (part_usage composite 'lugbolt' : 'Lugbolt'[unresolved]
          (multiplicity_range [4..5])))
      (part_usage 'wideRimWheel' : 'Wheel'[unresolved]
        (documentation)
        (part_usage composite 'lugbolt' : 'Lugbolt'[unresolved]
          (multiplicity_range [4..6])))
      (part_usage 'vehicle_C1' : 'Vehicle'[unresolved]
        (documentation)
        (part_usage composite 'frontAxleAssembly' : 'AxleAssembly'[unresolved]
          (part_usage composite 'frontWheel' :> 'VehicleUsages::narrowRimWheel'[part_usage]
            (multiplicity_range [2])
            (part_usage composite :>> 'VehicleUsages::narrowRimWheel::lugbolt'[part_usage]
              (multiplicity_range [4])
              (attribute_usage composite :>> 'tighteningTorque'[unresolved]
                (feature_value (=)))))
          (part_usage composite 'frontAxle' : 'Axle'[unresolved]))
        (part_usage composite 'rearAxleAssembly' : 'AxleAssembly'[unresolved]
          (part_usage composite 'rearWheel' :> 'VehicleUsages::wideRimWheel'[part_usage]
            (multiplicity_range [2])
            (part_usage composite :>> 'VehicleUsages::wideRimWheel::lugbolt'[part_usage]
              (multiplicity_range [6])
              (attribute_usage composite :>> 'tighteningTorque'[unresolved]
                (feature_value (=)))))
          (part_usage composite 'rearAxle' : 'Axle'[unresolved])))
      (part_usage 'vehicle_C2' :> 'VehicleUsages::vehicle_C1'[part_usage]
        (documentation)
        (part_usage composite :>> 'VehicleUsages::vehicle_C1::frontAxleAssembly'[part_usage]
          (part_usage composite 'leftFrontWheel' :> 'VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel'[part_usage]
            (feature_value (=)))
          (part_usage composite 'rightFrontWheel' :> 'VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel'[part_usage]
            (feature_value (=)))
          (interface_usage composite 'leftFrontMount' : 'Mounting'[unresolved]
            (connector_end 'frontAxle.leftMountingPoint')
            (connector_end 'leftFrontWheel.hub'))
          (interface_usage composite 'rightFrontMount' : 'Mounting'[unresolved]
            (connector_end 'frontAxle.rightMountingPoint')
            (connector_end 'rightFrontWheel.hub')))
        (part_usage composite 'rearAxleAssembly' :>> 'VehicleUsages::vehicle_C1::rearAxleAssembly'[part_usage]
          (part_usage composite 'leftRearWheel' :> 'VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel'[part_usage]
            (feature_value (=)))
          (part_usage composite 'rightRearWheel' :> 'VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel'[part_usage]
            (feature_value (=)))
          (interface_usage composite 'leftRearMount' : 'Mounting'[unresolved]
            (connector_end 'rearAxle.leftMountingPoint')
            (connector_end 'leftRearWheel.hub'))
          (interface_usage composite 'rightRearMount' : 'Mounting'[unresolved]
            (connector_end 'rearAxle.rightMountingPoint')
            (connector_end 'rightRearWheel.hub'))))
      (part_usage 'vehicle_C3' :> 'VehicleUsages::vehicle_C2'[part_usage]
        (documentation)
        (part_usage composite 'transmission' : 'Transmission'[unresolved]
          (port_usage composite 'drive' : 'DriveIF'[unresolved] ~ 'DriveIF'[unresolved]))
        (part_usage composite :>> 'VehicleUsages::vehicle_C2::rearAxleAssembly'[part_usage]
          (part_usage composite :>> 'VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle'[part_usage]
            (port_usage composite 'drive' : 'DriveIF'[unresolved])))
        (interface_usage composite 'driveShaft'
          (connector_end 'transDrive' :> 'VehicleUsages::vehicle_C3::transmission::drive'[port_usage])
          (connector_end 'axleDrive' :> 'drive'[port_usage])
          (flow_usage composite 'transDrive'))))))
~~~
