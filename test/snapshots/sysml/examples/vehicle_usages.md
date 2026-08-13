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
  (document "memory://snapshot/vehicle_usages.md"
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
        (range (start 8 16) (end 8 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 15) (end 10 36))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 13 1) (end 17 1))
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
        (code "unsupported_reference")
        (source "semantic")
        (range (start 33 30) (end 33 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 34 19) (end 34 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 35 25) (end 35 41))
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
        (code "unsupported_reference")
        (source "semantic")
        (range (start 41 29) (end 41 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 42 19) (end 42 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 43 25) (end 43 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 18) (end 46 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 50 25) (end 50 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 53 17) (end 53 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 54 31) (end 54 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 55 32) (end 55 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 57 3) (end 58 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 60 3) (end 61 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 64 34) (end 64 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 65 30) (end 65 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 66 31) (end 66 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 68 3) (end 69 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 71 3) (end 72 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 76 25) (end 76 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 21) (end 80 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 81 3) (end 81 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 84 17) (end 84 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 85 18) (end 85 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 86 4) (end 86 24))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 90 2) (end 94 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:264f69ead1d941fa32a0c5a242586c68803c9cff5e2c15381ad7f95a8f8c8aa8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::N") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::m") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VehicleDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::narrowRimWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::narrowRimWheel::lugbolt"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Lugbolt"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "narrowRimWheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "lugbolt"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "tighteningTorque"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Axle"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "wideRimWheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "lugbolt"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "tighteningTorque"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle_C1"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "frontAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::::leftFrontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "frontWheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::::rightFrontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "frontWheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicle_C1::rearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::leftRearWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "rearWheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::rightRearWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "rearWheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C3"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle_C2"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxle"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::wideRimWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::wideRimWheel::lugbolt"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Lugbolt"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleDefinitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::N")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::narrowRimWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::narrowRimWheel::lugbolt"))) (kind featureTyping) (ordinal 0))
      (authored-target "Lugbolt")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (kind subsetting) (ordinal 0))
      (authored-target "narrowRimWheel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "lugbolt")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "tighteningTorque")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Axle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (kind subsetting) (ordinal 0))
      (authored-target "wideRimWheel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "lugbolt")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "tighteningTorque")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle_C1")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "frontAxleAssembly")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::::leftFrontWheel"))) (kind subsetting) (ordinal 0))
      (authored-target "frontWheel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::::rightFrontWheel"))) (kind subsetting) (ordinal 0))
      (authored-target "frontWheel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind redefinition) (ordinal 0))
      (authored-target "vehicle_C1::rearAxleAssembly")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::leftRearWheel"))) (kind subsetting) (ordinal 0))
      (authored-target "rearWheel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::rightRearWheel"))) (kind subsetting) (ordinal 0))
      (authored-target "rearWheel")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C3"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle_C2")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxleAssembly")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::wideRimWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::wideRimWheel::lugbolt"))) (kind featureTyping) (ordinal 0))
      (authored-target "Lugbolt")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 8 16) (end 8 34)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 10 15) (end 10 36)) (probe (position 10 15))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleDefinitions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 6 16) (end 6 21)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "SI::N")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 7 16) (end 7 21)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SI::m")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 17 22) (end 17 27)) (probe (position 17 22))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::narrowRimWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 20 16) (end 20 23)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::narrowRimWheel::lugbolt"))) (kind featureTyping) (ordinal 0) (authored-target "Lugbolt")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 29 18) (end 29 25)) (probe (position 29 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 32 26) (end 32 38)) (probe (position 32 26))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 38 19) (end 38 23)) (probe (position 38 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 33 30) (end 33 44)) (probe (position 33 30))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::frontAxleAssembly::frontWheel"))) (kind subsetting) (ordinal 0) (authored-target "narrowRimWheel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 34 19) (end 34 26)) (probe (position 34 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "lugbolt")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 35 25) (end 35 41)) (probe (position 35 25))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "tighteningTorque")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 40 25) (end 40 37)) (probe (position 40 25))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "AxleAssembly")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 46 18) (end 46 22)) (probe (position 46 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearAxle"))) (kind featureTyping) (ordinal 0) (authored-target "Axle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 41 29) (end 41 41)) (probe (position 41 29))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C1::rearAxleAssembly::rearWheel"))) (kind subsetting) (ordinal 0) (authored-target "wideRimWheel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 42 19) (end 42 26)) (probe (position 42 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "lugbolt")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 43 25) (end 43 41)) (probe (position 43 25))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "tighteningTorque")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 50 25) (end 50 35)) (probe (position 50 25))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2"))) (kind subsetting) (ordinal 0) (authored-target "vehicle_C1")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 53 17) (end 53 34)) (probe (position 53 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "frontAxleAssembly")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 54 31) (end 54 41)) (probe (position 54 31))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::::leftFrontWheel"))) (kind subsetting) (ordinal 0) (authored-target "frontWheel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 55 32) (end 55 42)) (probe (position 55 32))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::::rightFrontWheel"))) (kind subsetting) (ordinal 0) (authored-target "frontWheel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 64 34) (end 64 62)) (probe (position 64 34))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly"))) (kind redefinition) (ordinal 0) (authored-target "vehicle_C1::rearAxleAssembly")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 65 30) (end 65 39)) (probe (position 65 30))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::leftRearWheel"))) (kind subsetting) (ordinal 0) (authored-target "rearWheel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 66 31) (end 66 40)) (probe (position 66 31))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C2::rearAxleAssembly::rightRearWheel"))) (kind subsetting) (ordinal 0) (authored-target "rearWheel")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 76 25) (end 76 35)) (probe (position 76 25))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C3"))) (kind subsetting) (ordinal 0) (authored-target "vehicle_C2")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 84 17) (end 84 33)) (probe (position 84 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 85 18) (end 85 26)) (probe (position 85 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "rearAxle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 80 21) (end 80 33)) (probe (position 80 21))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::vehicle_C3::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 23 20) (end 23 25)) (probe (position 23 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::wideRimWheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_usages.md") (range (start 26 16) (end 26 23)) (probe (position 26 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_usages.md") (qualified-name "VehicleUsages::wideRimWheel::lugbolt"))) (kind featureTyping) (ordinal 0) (authored-target "Lugbolt")
      (outcome (status unresolved)))
  )
)
~~~
