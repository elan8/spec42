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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VehicleUsages"))) (name "VehicleUsages") (declared-name "VehicleUsages")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleUsages::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleUsages::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleUsages::N"))) (name "N") (declared-name "N"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleUsages::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleUsages::m"))) (name "m") (declared-name "m"))
        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (name "narrowRimWheel") (declared-name "narrowRimWheel") (declared (properties (ordered false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::_documentation"))) (name ""))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::lugbolt"))) (name "lugbolt") (declared-name "lugbolt") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 5) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (name "vehicle_C1") (declared-name "vehicle_C1") (declared (properties (ordered false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::_documentation"))) (name ""))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontAxle"))) (name "frontAxle") (declared-name "frontAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (name "frontWheel") (declared-name "frontWheel") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (name "lugbolt") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))) (name "tighteningTorque") (declared-name "tighteningTorque") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "T1")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))) (role feature-value))))
                      )
                    )
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared-name "rearAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (name "rearWheel") (declared-name "rearWheel") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (name "lugbolt") (declared (properties (ordered false)) (multiplicity (lower 6) (upper 6) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))) (name "tighteningTorque") (declared-name "tighteningTorque") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "T2")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))) (role feature-value))))
                      )
                    )
                  )
                )
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (name "vehicle_C2") (declared-name "vehicle_C2") (declared (properties (ordered false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::_documentation"))) (name ""))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (name "frontAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::leftFrontWheel"))) (name "leftFrontWheel") (declared-name "leftFrontWheel") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::rightFrontWheel"))) (name "rightFrontWheel") (declared-name "rightFrontWheel") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::leftRearWheel"))) (name "leftRearWheel") (declared-name "leftRearWheel") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::rightRearWheel"))) (name "rightRearWheel") (declared-name "rightRearWheel") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (name "vehicle_C3") (declared-name "vehicle_C3") (declared (properties (ordered false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::_documentation"))) (name ""))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle::drive"))) (name "drive") (declared-name "drive") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission::drive"))) (name "drive") (declared-name "drive") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (name "wideRimWheel") (declared-name "wideRimWheel") (declared (properties (ordered false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel::_documentation"))) (name ""))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel::lugbolt"))) (name "lugbolt") (declared-name "lugbolt") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 6) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleUsages::_documentation"))) (to (node (document "d0") (qualified-name "VehicleUsages"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::_documentation"))) (to (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::_documentation"))) (to (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::_documentation"))) (to (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::_documentation"))) (to (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel::_documentation"))) (to (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (to (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (to (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (connection (status pending-expression) (document "d0") (source-expression "frontAxle::leftMountingPoint") (target-expression "leftFrontWheel::hub") (container-prefix "VehicleUsages::vehicle_C2::frontAxleAssembly") (interface-usage true) (interface-type "Mounting"))
    (connection (status pending-expression) (document "d0") (source-expression "frontAxle::rightMountingPoint") (target-expression "rightFrontWheel::hub") (container-prefix "VehicleUsages::vehicle_C2::frontAxleAssembly") (interface-usage true) (interface-type "Mounting"))
    (connection (status pending-expression) (document "d0") (source-expression "rearAxle::leftMountingPoint") (target-expression "leftRearWheel::hub") (container-prefix "VehicleUsages::vehicle_C2::rearAxleAssembly") (interface-usage true) (interface-type "Mounting"))
    (connection (status pending-expression) (document "d0") (source-expression "rearAxle::rightMountingPoint") (target-expression "rightRearWheel::hub") (container-prefix "VehicleUsages::vehicle_C2::rearAxleAssembly") (interface-usage true) (interface-type "Mounting"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicle_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 1) (end 6 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 1) (end 8 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 1) (end 10 37))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 13 1) (end 13 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 22) (end 17 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 16) (end 20 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 20) (end 23 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 16) (end 26 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 18) (end 29 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 26) (end 32 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 34 4) (end 34 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 35 5) (end 35 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 19) (end 38 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 25) (end 40 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 42 4) (end 42 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 43 5) (end 43 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 18) (end 46 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 53 2) (end 53 377))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 58 4) (end 58 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 58 4) (end 58 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 58 4) (end 58 31))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 61 4) (end 61 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 61 4) (end 61 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 61 4) (end 61 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 64 2) (end 64 390))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 69 4) (end 69 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 69 4) (end 69 30))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 72 4) (end 72 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 72 4) (end 72 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 21) (end 80 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 3) (end 81 24))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 81 3) (end 81 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 84 2) (end 84 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 85 3) (end 85 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 4) (end 86 24))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 86 4) (end 86 24))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 90 2) (end 90 184))
      )
    )
  )
)
~~~
