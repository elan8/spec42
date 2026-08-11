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
    (element (id (node (document "d0") (qualified-name "VehicleUsages"))) (kind "package") (name "VehicleUsages") (declared-name "VehicleUsages"))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Import) (visibility "public") (import (reference "VehicleDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::N"))) (kind "import") (name "N") (declared-name "N") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::N") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleUsages"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::m"))) (kind "import") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::m") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (kind "part") (name "narrowRimWheel") (declared-name "narrowRimWheel") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::lugbolt"))) (kind "part") (name "lugbolt") (declared-name "lugbolt") (parent (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Lugbolt")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (kind "part") (name "vehicle_C1") (declared-name "vehicle_C1") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontAxle"))) (kind "part") (name "frontAxle") (declared-name "frontAxle") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (kind "part") (name "frontWheel") (declared-name "frontWheel") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "narrowRimWheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (kind "part") (name "lugbolt") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "lugbolt")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))) (kind "attribute") (name "tighteningTorque") (declared-name "tighteningTorque") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "tighteningTorque")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (declared-name "rearAxle") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Axle")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (kind "part") (name "rearWheel") (declared-name "rearWheel") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "wideRimWheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (kind "part") (name "lugbolt") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "lugbolt")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))) (kind "attribute") (name "tighteningTorque") (declared-name "tighteningTorque") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "tighteningTorque")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (kind "part") (name "vehicle_C2") (declared-name "vehicle_C2") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle_C1")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "frontAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::leftFrontWheel"))) (kind "part") (name "leftFrontWheel") (declared-name "leftFrontWheel") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::rightFrontWheel"))) (kind "part") (name "rightFrontWheel") (declared-name "rightFrontWheel") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "frontWheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "vehicle_C1::rearAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::leftRearWheel"))) (kind "part") (name "leftRearWheel") (declared-name "leftRearWheel") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::rightRearWheel"))) (kind "part") (name "rightRearWheel") (declared-name "rightRearWheel") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "rearWheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (kind "part") (name "vehicle_C3") (declared-name "vehicle_C3") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle_C2")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxle")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle::drive"))) (kind "port") (name "drive") (declared-name "drive") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DriveIF")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission::drive"))) (kind "port") (name "drive") (declared-name "drive") (parent (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "~DriveIF")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (kind "part") (name "wideRimWheel") (declared-name "wideRimWheel") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel::lugbolt"))) (kind "part") (name "lugbolt") (declared-name "lugbolt") (parent (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Lugbolt")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleDefinitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::N"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::N") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::m"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::m") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::lugbolt"))) (kind featureTyping) (ordinal 0)) (authored-target "Lugbolt") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (kind subsetting) (ordinal 0)) (authored-target "narrowRimWheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))) (kind redefinition) (ordinal 0)) (authored-target "lugbolt") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))) (kind redefinition) (ordinal 0)) (authored-target "tighteningTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleAssembly") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0)) (authored-target "Axle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (kind subsetting) (ordinal 0)) (authored-target "wideRimWheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))) (kind redefinition) (ordinal 0)) (authored-target "lugbolt") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))) (kind redefinition) (ordinal 0)) (authored-target "tighteningTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle_C1") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "frontAxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind connectionSource) (ordinal 0)) (authored-target "frontAxle::leftMountingPoint") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind connectionSource) (ordinal 1)) (authored-target "frontAxle::rightMountingPoint") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind connectionTarget) (ordinal 0)) (authored-target "leftFrontWheel::hub") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))) (kind connectionTarget) (ordinal 1)) (authored-target "rightFrontWheel::hub") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::leftFrontWheel"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::rightFrontWheel"))) (kind subsetting) (ordinal 0)) (authored-target "frontWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "vehicle_C1::rearAxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind connectionSource) (ordinal 0)) (authored-target "rearAxle::leftMountingPoint") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind connectionSource) (ordinal 1)) (authored-target "rearAxle::rightMountingPoint") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind connectionTarget) (ordinal 0)) (authored-target "leftRearWheel::hub") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind connectionTarget) (ordinal 1)) (authored-target "rightRearWheel::hub") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::leftRearWheel"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::rightRearWheel"))) (kind subsetting) (ordinal 0)) (authored-target "rearWheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle_C2") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxle") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle::drive"))) (kind featureTyping) (ordinal 0)) (authored-target "DriveIF") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission::drive"))) (kind featureTyping) (ordinal 0)) (authored-target "~DriveIF") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel::lugbolt"))) (kind featureTyping) (ordinal 0)) (authored-target "Lugbolt") (outcome (status unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 38 19) (end 38 23)) (probe (position 38 19))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "Axle")
        (range (start 38 19) (end 38 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 46 18) (end 46 22)) (probe (position 46 18))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle"))
        (kind featureTyping) (ordinal 0) (authored-target "Axle")
        (range (start 46 18) (end 46 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 21)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::N"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::N")
        (range (start 6 16) (end 6 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 21)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::m"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::m")
        (range (start 7 16) (end 7 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 22) (end 17 27)) (probe (position 17 22))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 17 22) (end 17 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 23 20) (end 23 25)) (probe (position 23 20))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 23 20) (end 23 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 16) (end 20 23)) (probe (position 20 16))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::narrowRimWheel::lugbolt"))
        (kind featureTyping) (ordinal 0) (authored-target "Lugbolt")
        (range (start 20 16) (end 20 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 16) (end 26 23)) (probe (position 26 16))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::wideRimWheel::lugbolt"))
        (kind featureTyping) (ordinal 0) (authored-target "Lugbolt")
        (range (start 26 16) (end 26 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 29 18) (end 29 25)) (probe (position 29 18))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 29 18) (end 29 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 34 19) (end 34 26)) (probe (position 34 19))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt"))
        (kind redefinition) (ordinal 0) (authored-target "lugbolt")
        (range (start 34 19) (end 34 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt") (range (start 34 4) (end 34 85)))
        )
      )
    )
    (query (range (start 42 19) (end 42 26)) (probe (position 42 19))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt"))
        (kind redefinition) (ordinal 0) (authored-target "lugbolt")
        (range (start 42 19) (end 42 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt") (range (start 42 4) (end 42 85)))
        )
      )
    )
    (query (range (start 85 18) (end 85 26)) (probe (position 85 18))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle"))
        (kind redefinition) (ordinal 0) (authored-target "rearAxle")
        (range (start 85 18) (end 85 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly::rearAxle") (range (start 85 3) (end 85 58)))
        )
      )
    )
    (query (range (start 65 30) (end 65 39)) (probe (position 65 30))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::leftRearWheel"))
        (kind subsetting) (ordinal 0) (authored-target "rearWheel")
        (range (start 65 30) (end 65 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 66 31) (end 66 40)) (probe (position 66 31))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::rightRearWheel"))
        (kind subsetting) (ordinal 0) (authored-target "rearWheel")
        (range (start 66 31) (end 66 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 50 25) (end 50 35)) (probe (position 50 25))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle_C1")
        (range (start 50 25) (end 50 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C1") (range (start 29 1) (end 29 513)))
        )
      )
    )
    (query (range (start 54 31) (end 54 41)) (probe (position 54 31))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::leftFrontWheel"))
        (kind subsetting) (ordinal 0) (authored-target "frontWheel")
        (range (start 54 31) (end 54 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 55 32) (end 55 42)) (probe (position 55 32))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly::rightFrontWheel"))
        (kind subsetting) (ordinal 0) (authored-target "frontWheel")
        (range (start 55 32) (end 55 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 76 25) (end 76 35)) (probe (position 76 25))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle_C2")
        (range (start 76 25) (end 76 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C2") (range (start 50 1) (end 50 879)))
        )
      )
    )
    (query (range (start 32 26) (end 32 38)) (probe (position 32 26))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 32 26) (end 32 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 25) (end 40 37)) (probe (position 40 25))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
        (range (start 40 25) (end 40 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 41 29) (end 41 41)) (probe (position 41 29))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))
        (kind subsetting) (ordinal 0) (authored-target "wideRimWheel")
        (range (start 41 29) (end 41 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::wideRimWheel") (range (start 23 1) (end 23 126)))
        )
      )
    )
    (query (range (start 80 21) (end 80 33)) (probe (position 80 21))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C3::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 80 21) (end 80 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 30) (end 33 44)) (probe (position 33 30))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))
        (kind subsetting) (ordinal 0) (authored-target "narrowRimWheel")
        (range (start 33 30) (end 33 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::narrowRimWheel") (range (start 17 1) (end 17 129)))
        )
      )
    )
    (query (range (start 8 16) (end 8 31)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarFunctions::*")
        (range (start 8 16) (end 8 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 35 25) (end 35 41)) (probe (position 35 25))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque"))
        (kind redefinition) (ordinal 0) (authored-target "tighteningTorque")
        (range (start 35 25) (end 35 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel::lugbolt::tighteningTorque") (range (start 35 5) (end 35 47)))
        )
      )
    )
    (query (range (start 43 25) (end 43 41)) (probe (position 43 25))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque"))
        (kind redefinition) (ordinal 0) (authored-target "tighteningTorque")
        (range (start 43 25) (end 43 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel::lugbolt::tighteningTorque") (range (start 43 5) (end 43 47)))
        )
      )
    )
    (query (range (start 84 17) (end 84 33)) (probe (position 84 17))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly"))
        (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
        (range (start 84 17) (end 84 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C3::rearAxleAssembly") (range (start 84 2) (end 84 98)))
        )
      )
    )
    (query (range (start 53 17) (end 53 34)) (probe (position 53 17))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))
        (kind redefinition) (ordinal 0) (authored-target "frontAxleAssembly")
        (range (start 53 17) (end 53 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly") (range (start 53 2) (end 53 377)))
        )
      )
    )
    (query (range (start 69 34) (end 69 51)) (probe (position 69 34))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))
        (kind connectionTarget) (ordinal 0) (authored-target "leftRearWheel::hub")
        (range (start 69 34) (end 69 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 15) (end 10 33)) (probe (position 10 15))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleDefinitions::*")
        (range (start 10 15) (end 10 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 35) (end 58 53)) (probe (position 58 35))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))
        (kind connectionTarget) (ordinal 0) (authored-target "leftFrontWheel::hub")
        (range (start 58 35) (end 58 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 72 35) (end 72 53)) (probe (position 72 35))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))
        (kind connectionTarget) (ordinal 1) (authored-target "rightRearWheel::hub")
        (range (start 72 35) (end 72 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 36) (end 61 55)) (probe (position 61 36))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))
        (kind connectionTarget) (ordinal 1) (authored-target "rightFrontWheel::hub")
        (range (start 61 36) (end 61 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 69 4) (end 69 30)) (probe (position 69 4))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))
        (kind connectionSource) (ordinal 0) (authored-target "rearAxle::leftMountingPoint")
        (range (start 69 4) (end 69 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 4) (end 58 31)) (probe (position 58 4))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))
        (kind connectionSource) (ordinal 0) (authored-target "frontAxle::leftMountingPoint")
        (range (start 58 4) (end 58 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 72 4) (end 72 31)) (probe (position 72 4))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))
        (kind connectionSource) (ordinal 1) (authored-target "rearAxle::rightMountingPoint")
        (range (start 72 4) (end 72 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 4) (end 61 32)) (probe (position 61 4))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::frontAxleAssembly"))
        (kind connectionSource) (ordinal 1) (authored-target "frontAxle::rightMountingPoint")
        (range (start 61 4) (end 61 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 64 34) (end 64 62)) (probe (position 64 34))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))
        (kind redefinition) (ordinal 0) (authored-target "vehicle_C1::rearAxleAssembly")
        (range (start 64 34) (end 64 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly") (range (start 40 2) (end 40 205)))
        )
      )
    )
  )
)
~~~
