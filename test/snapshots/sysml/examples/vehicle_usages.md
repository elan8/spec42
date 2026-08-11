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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 15) (end 10 33))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 18) (end 46 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 31) (end 54 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 32) (end 55 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 4) (end 58 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 35) (end 58 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 4) (end 61 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 36) (end 61 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 30) (end 65 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 66 31) (end 66 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 4) (end 69 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 34) (end 69 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 72 4) (end 72 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 72 35) (end 72 53))
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
        (severity warning)
        (code "unresolved_type_reference")
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "012e3b834827a798044d10e1f657b23cda4495f8458bb0f977df53e8569a3922") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleUsages"))) (kind "package") (name "VehicleUsages") (declared-name "VehicleUsages") (range (start (line 0) (character 0)) (end (line 0) (character 2446))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 1)) (end (line 8) (character 35))) (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 31))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 10) (character 1)) (end (line 10) (character 37))) (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Import) (visibility "public") (import (reference "VehicleDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 10) (character 15)) (end (line 10) (character 33))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::N"))) (kind "import") (name "N") (declared-name "N") (range (start (line 6) (character 1)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::N") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 21))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2446))) (parent (node (document "d0") (qualified-name "VehicleUsages"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::m"))) (kind "import") (name "m") (declared-name "m") (range (start (line 7) (character 1)) (end (line 7) (character 22))) (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::m") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 21))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (kind "part") (name "narrowRimWheel") (declared-name "narrowRimWheel") (range (start (line 17) (character 1)) (end (line 17) (character 129))) (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 17) (character 22)) (end (line 17) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::_documentation"))) (kind "documentation") (name "") (range (start (line 17) (character 1)) (end (line 17) (character 129))) (parent (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::lugbolt"))) (kind "part") (name "lugbolt") (declared-name "lugbolt") (range (start (line 20) (character 2)) (end (line 20) (character 30))) (parent (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Lugbolt") (range (start (line 20) (character 16)) (end (line 20) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (kind "part") (name "vehicle_C1") (declared-name "vehicle_C1") (range (start (line 29) (character 1)) (end (line 29) (character 513))) (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 29) (character 18)) (end (line 29) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::_documentation"))) (kind "documentation") (name "") (range (start (line 29) (character 1)) (end (line 29) (character 513))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (range (start (line 32) (character 2)) (end (line 32) (character 207))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 32) (character 26)) (end (line 32) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontAxle"))) (kind "part") (name "frontAxle") (declared-name "frontAxle") (range (start (line 38) (character 3)) (end (line 38) (character 24))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle") (range (start (line 38) (character 19)) (end (line 38) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (kind "part") (name "frontWheel") (declared-name "frontWheel") (range (start (line 33) (character 3)) (end (line 33) (character 137))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "narrowRimWheel") (range (start (line 33) (character 30)) (end (line 33) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (kind "part") (name "lugbolt") (range (start (line 34) (character 4)) (end (line 34) (character 85))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "lugbolt") (range (start (line 34) (character 19)) (end (line 34) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))) (kind "attribute") (name "tighteningTorque") (declared-name "tighteningTorque") (range (start (line 35) (character 5)) (end (line 35) (character 47))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "tighteningTorque") (range (start (line 35) (character 25)) (end (line 35) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 40) (character 2)) (end (line 40) (character 205))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly") (range (start (line 40) (character 25)) (end (line 40) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (range (start (line 46) (character 3)) (end (line 46) (character 23))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle") (range (start (line 46) (character 18)) (end (line 46) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (range (start (line 41) (character 3)) (end (line 41) (character 134))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "wideRimWheel") (range (start (line 41) (character 29)) (end (line 41) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (kind "part") (name "lugbolt") (range (start (line 42) (character 4)) (end (line 42) (character 85))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "lugbolt") (range (start (line 42) (character 19)) (end (line 42) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))) (kind "attribute") (name "tighteningTorque") (declared-name "tighteningTorque") (range (start (line 43) (character 5)) (end (line 43) (character 47))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "tighteningTorque") (range (start (line 43) (character 25)) (end (line 43) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (kind "part") (name "vehicle_C2") (declared-name "vehicle_C2") (range (start (line 50) (character 1)) (end (line 50) (character 879))) (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle_C1") (range (start (line 50) (character 25)) (end (line 50) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::_documentation"))) (kind "documentation") (name "") (range (start (line 50) (character 1)) (end (line 50) (character 879))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (range (start (line 53) (character 2)) (end (line 53) (character 377))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "frontAxleAssembly") (range (start (line 53) (character 17)) (end (line 53) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::leftFrontWheel"))) (kind "part") (name "leftFrontWheel") (declared-name "leftFrontWheel") (range (start (line 54) (character 3)) (end (line 54) (character 59))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel") (range (start (line 54) (character 31)) (end (line 54) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::rightFrontWheel"))) (kind "part") (name "rightFrontWheel") (declared-name "rightFrontWheel") (range (start (line 55) (character 3)) (end (line 55) (character 60))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel") (range (start (line 55) (character 32)) (end (line 55) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 64) (character 2)) (end (line 64) (character 390))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "vehicle_C1::rearAxleAssembly") (range (start (line 64) (character 34)) (end (line 64) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::leftRearWheel"))) (kind "part") (name "leftRearWheel") (declared-name "leftRearWheel") (range (start (line 65) (character 3)) (end (line 65) (character 56))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel") (range (start (line 65) (character 30)) (end (line 65) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::rightRearWheel"))) (kind "part") (name "rightRearWheel") (declared-name "rightRearWheel") (range (start (line 66) (character 3)) (end (line 66) (character 57))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel") (range (start (line 66) (character 31)) (end (line 66) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (kind "part") (name "vehicle_C3") (declared-name "vehicle_C3") (range (start (line 76) (character 1)) (end (line 76) (character 486))) (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle_C2") (range (start (line 76) (character 25)) (end (line 76) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::_documentation"))) (kind "documentation") (name "") (range (start (line 76) (character 1)) (end (line 76) (character 486))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (range (start (line 84) (character 2)) (end (line 84) (character 98))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxleAssembly") (range (start (line 84) (character 17)) (end (line 84) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (range (start (line 85) (character 3)) (end (line 85) (character 58))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxle") (range (start (line 85) (character 18)) (end (line 85) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle::drive"))) (kind "port") (name "drive") (declared-name "drive") (range (start (line 86) (character 4)) (end (line 86) (character 24))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DriveIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 80) (character 2)) (end (line 80) (character 64))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 80) (character 21)) (end (line 80) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission::drive"))) (kind "port") (name "drive") (declared-name "drive") (range (start (line 81) (character 3)) (end (line 81) (character 24))) (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "~DriveIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (kind "part") (name "wideRimWheel") (declared-name "wideRimWheel") (range (start (line 23) (character 1)) (end (line 23) (character 126))) (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 23) (character 20)) (end (line 23) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel::_documentation"))) (kind "documentation") (name "") (range (start (line 23) (character 1)) (end (line 23) (character 126))) (parent (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel::lugbolt"))) (kind "part") (name "lugbolt") (declared-name "lugbolt") (range (start (line 26) (character 2)) (end (line 26) (character 30))) (parent (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Lugbolt") (range (start (line 26) (character 16)) (end (line 26) (character 23)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarFunctions::*") (range (start (line 8) (character 16)) (end (line 8) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleDefinitions::*") (range (start (line 10) (character 15)) (end (line 10) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::N"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::N") (range (start (line 6) (character 16)) (end (line 6) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::m"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::m") (range (start (line 7) (character 16)) (end (line 7) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 17) (character 22)) (end (line 17) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::lugbolt"))) (kind featureTyping) (ordinal 0)) (authored-target "Lugbolt") (range (start (line 20) (character 16)) (end (line 20) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 29) (character 18)) (end (line 29) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 32) (character 26)) (end (line 32) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (range (start (line 38) (character 19)) (end (line 38) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (kind subsetting) (ordinal 0)) (authored-target "narrowRimWheel") (range (start (line 33) (character 30)) (end (line 33) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (kind redefinition) (ordinal 0)) (authored-target "lugbolt") (range (start (line 34) (character 19)) (end (line 34) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))) (kind redefinition) (ordinal 0)) (authored-target "tighteningTorque") (range (start (line 35) (character 25)) (end (line 35) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (range (start (line 40) (character 25)) (end (line 40) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (range (start (line 46) (character 18)) (end (line 46) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (kind subsetting) (ordinal 0)) (authored-target "wideRimWheel") (range (start (line 41) (character 29)) (end (line 41) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (kind redefinition) (ordinal 0)) (authored-target "lugbolt") (range (start (line 42) (character 19)) (end (line 42) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))) (kind redefinition) (ordinal 0)) (authored-target "tighteningTorque") (range (start (line 43) (character 25)) (end (line 43) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle_C1") (range (start (line 50) (character 25)) (end (line 50) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxleAssembly") (range (start (line 53) (character 17)) (end (line 53) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind connectionSource) (ordinal 0)) (authored-target "frontAxle::leftMountingPoint") (range (start (line 58) (character 4)) (end (line 58) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind connectionSource) (ordinal 1)) (authored-target "frontAxle::rightMountingPoint") (range (start (line 61) (character 4)) (end (line 61) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind connectionTarget) (ordinal 0)) (authored-target "leftFrontWheel::hub") (range (start (line 58) (character 35)) (end (line 58) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind connectionTarget) (ordinal 1)) (authored-target "rightFrontWheel::hub") (range (start (line 61) (character 36)) (end (line 61) (character 55))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::leftFrontWheel"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (range (start (line 54) (character 31)) (end (line 54) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::rightFrontWheel"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (range (start (line 55) (character 32)) (end (line 55) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "vehicle_C1::rearAxleAssembly") (range (start (line 64) (character 34)) (end (line 64) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind connectionSource) (ordinal 0)) (authored-target "rearAxle::leftMountingPoint") (range (start (line 69) (character 4)) (end (line 69) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind connectionSource) (ordinal 1)) (authored-target "rearAxle::rightMountingPoint") (range (start (line 72) (character 4)) (end (line 72) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind connectionTarget) (ordinal 0)) (authored-target "leftRearWheel::hub") (range (start (line 69) (character 34)) (end (line 69) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind connectionTarget) (ordinal 1)) (authored-target "rightRearWheel::hub") (range (start (line 72) (character 35)) (end (line 72) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::leftRearWheel"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (range (start (line 65) (character 30)) (end (line 65) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::rightRearWheel"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (range (start (line 66) (character 31)) (end (line 66) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle_C2") (range (start (line 76) (character 25)) (end (line 76) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxleAssembly") (range (start (line 84) (character 17)) (end (line 84) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxle") (range (start (line 85) (character 18)) (end (line 85) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle::drive"))) (kind featureTyping) (ordinal 0)) (authored-target "DriveIF") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 80) (character 21)) (end (line 80) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission::drive"))) (kind featureTyping) (ordinal 0)) (authored-target "~DriveIF") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 23) (character 20)) (end (line 23) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel::lugbolt"))) (kind featureTyping) (ordinal 0)) (authored-target "Lugbolt") (range (start (line 26) (character 16)) (end (line 26) (character 23))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (target (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (target (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
