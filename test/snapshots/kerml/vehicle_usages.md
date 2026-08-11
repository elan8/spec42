# META
~~~ini
description=KerML Vehicle: VehicleUsages
type=file
~~~
# SOURCE
~~~kerml
package VehicleUsages {
	doc
	/*
	 * Example usages of elements from the vehicle definitions model.
	 */
	 
	private import VehicleDefinitions::*;

	/* VALUES */
		 
	feature T1 = 10.0;
	feature T2 = 20.0;
	
	/* PARTS */	
	
	feature narrowRimWheel: Wheel {
		doc /* Narrow-rim wheel configuration with 4 to 5 lugbolts. */
		composite lugbolt: Lugbolt[4..5];
	}
		
	feature wideRimWheel: Wheel {
		doc /* Wide-rim wheel configuration with 4 to 6 lugbolts. */
		composite lugbolt: Lugbolt[4..6];
	}

	feature vehicle_C1: Vehicle {
		doc /* Basic Vehicle configuration showing a part hierarchy. */
		composite frontAxleAssembly: AxleAssembly {
			composite frontWheel[2] redefines narrowRimWheel {
				composite lugbolt[4] redefines narrowRimWheel::lugbolt {
					feature tighteningTorque redefines Lugbolt::tighteningTorque = T1;
				}
			}
			composite frontAxle: Axle;
		}		
		composite rearAxleAssembly: VehicleDefinitions::AxleAssembly {
			composite rearWheel[2] redefines wideRimWheel {
				composite lugbolt[6] redefines wideRimWheel::lugbolt {
					feature tighteningTorque redefines Lugbolt::tighteningTorque = T2;
				}
			}
			composite rearAxle: Axle;			
		}
	}
	
	feature vehicle_C2 subsets vehicle_C1 {
		doc /* Specialized configuration with part-specific ports. */
		composite frontAxleAssembly redefines vehicle_C1::frontAxleAssembly {
			composite leftFrontWheel subsets vehicle_C1::frontAxleAssembly::frontWheel = vehicle_C1::frontAxleAssembly::frontWheel#(1) {
				composite hub: VehicleDefinitions::WheelHubIF;
			}
			composite rightFrontWheel subsets vehicle_C1::frontAxleAssembly::frontWheel = vehicle_C1::frontAxleAssembly::frontWheel#(2) {
				feature hub: VehicleDefinitions::WheelHubIF;
			}
			
			composite frontAxle redefines vehicle_C1::frontAxleAssembly::frontAxle {
				composite leftMountingPoint: AxleMountIF;
				composite rightMountingPoint: AxleMountIF;
			}
		
			connector leftFrontMount: Mounting from 
				frontAxle.leftMountingPoint to leftFrontWheel.hub;
				
			connector rightFrontMount: Mounting from 
				frontAxle.rightMountingPoint to rightFrontWheel.hub;
		}
		
		composite rearAxleAssembly redefines vehicle_C1::rearAxleAssembly {
			composite leftRearWheel subsets vehicle_C1::rearAxleAssembly::rearWheel = vehicle_C1::rearAxleAssembly::rearWheel#(1) {
				feature hub: WheelHubIF;
			}
			composite rightRearWheel subsets vehicle_C1::rearAxleAssembly::rearWheel = vehicle_C1::rearAxleAssembly::rearWheel#(2) {
				feature hub: WheelHubIF;
			}

			composite rearAxle redefines vehicle_C1::rearAxleAssembly::rearAxle {
				feature leftMountingPoint: AxleMountIF;
				feature rightMountingPoint: AxleMountIF;
			}
			
			connector leftRearMount: Mounting from 
				rearAxle.leftMountingPoint to leftRearWheel.hub;
				
			connector rightRearMount: Mounting from 
				rearAxle.rightMountingPoint to rightRearWheel.hub;
		}		
	}
	
	feature vehicle_C3 subsets vehicle_C2 {
		doc /* Further specialized configuration with a connector to a deeply-nested feature. */
		composite transmission: Transmission {
			out feature drive: DriveIF;
		}
		
		composite rearAxleAssembly redefines vehicle_C2::rearAxleAssembly {
			composite rearAxle redefines vehicle_C2::rearAxleAssembly::rearAxle {
				in feature drive: DriveIF;
			}
		}
		
		connector driveShaft from 
			transmission.drive to rearAxleAssembly.rearAxle.drive;			
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
        (range (start 6 16) (end 6 34))
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
	 
	private import VehicleDefinitions::*;

	/* VALUES */
		 
	feature T1 = 10.0;
	feature T2 = 20.0;
	
	/* PARTS */	
	
	feature narrowRimWheel: Wheel {
		doc /* Narrow-rim wheel configuration with 4 to 5 lugbolts. */
		composite lugbolt: Lugbolt[4..5];
	}
		
	feature wideRimWheel: Wheel {
		doc /* Wide-rim wheel configuration with 4 to 6 lugbolts. */
		composite lugbolt: Lugbolt[4..6];
	}

	feature vehicle_C1: Vehicle {
		doc /* Basic Vehicle configuration showing a part hierarchy. */
		composite frontAxleAssembly: AxleAssembly {
			composite frontWheel[2] redefines narrowRimWheel {
				composite lugbolt[4] redefines narrowRimWheel::lugbolt {
					feature tighteningTorque redefines Lugbolt::tighteningTorque = T1;
				}
			}
			composite frontAxle: Axle;
		}		
		composite rearAxleAssembly: VehicleDefinitions::AxleAssembly {
			composite rearWheel[2] redefines wideRimWheel {
				composite lugbolt[6] redefines wideRimWheel::lugbolt {
					feature tighteningTorque redefines Lugbolt::tighteningTorque = T2;
				}
			}
			composite rearAxle: Axle;			
		}
	}
	
	feature vehicle_C2 subsets vehicle_C1 {
		doc /* Specialized configuration with part-specific ports. */
		composite frontAxleAssembly redefines vehicle_C1::frontAxleAssembly {
			composite leftFrontWheel subsets vehicle_C1::frontAxleAssembly::frontWheel = vehicle_C1::frontAxleAssembly::frontWheel#(1) {
				composite hub: VehicleDefinitions::WheelHubIF;
			}
			composite rightFrontWheel subsets vehicle_C1::frontAxleAssembly::frontWheel = vehicle_C1::frontAxleAssembly::frontWheel#(2) {
				feature hub: VehicleDefinitions::WheelHubIF;
			}
			
			composite frontAxle redefines vehicle_C1::frontAxleAssembly::frontAxle {
				composite leftMountingPoint: AxleMountIF;
				composite rightMountingPoint: AxleMountIF;
			}
		
			connector leftFrontMount: Mounting from 
				frontAxle.leftMountingPoint to leftFrontWheel.hub;
				
			connector rightFrontMount: Mounting from 
				frontAxle.rightMountingPoint to rightFrontWheel.hub;
		}
		
		composite rearAxleAssembly redefines vehicle_C1::rearAxleAssembly {
			composite leftRearWheel subsets vehicle_C1::rearAxleAssembly::rearWheel = vehicle_C1::rearAxleAssembly::rearWheel#(1) {
				feature hub: WheelHubIF;
			}
			composite rightRearWheel subsets vehicle_C1::rearAxleAssembly::rearWheel = vehicle_C1::rearAxleAssembly::rearWheel#(2) {
				feature hub: WheelHubIF;
			}

			composite rearAxle redefines vehicle_C1::rearAxleAssembly::rearAxle {
				feature leftMountingPoint: AxleMountIF;
				feature rightMountingPoint: AxleMountIF;
			}
			
			connector leftRearMount: Mounting from 
				rearAxle.leftMountingPoint to leftRearWheel.hub;
				
			connector rightRearMount: Mounting from 
				rearAxle.rightMountingPoint to rightRearWheel.hub;
		}		
	}
	
	feature vehicle_C3 subsets vehicle_C2 {
		doc /* Further specialized configuration with a connector to a deeply-nested feature. */
		composite transmission: Transmission {
			out feature drive: DriveIF;
		}
		
		composite rearAxleAssembly redefines vehicle_C2::rearAxleAssembly {
			composite rearAxle redefines vehicle_C2::rearAxleAssembly::rearAxle {
				in feature drive: DriveIF;
			}
		}
		
		connector driveShaft from 
			transmission.drive to rearAxleAssembly.rearAxle.drive;			
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "41aa674f97203d346bca3a8e960ed2f49799764a91cf2f129861abc7d531cd26") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleUsages"))) (kind "package") (name "VehicleUsages") (declared-name "VehicleUsages"))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleUsages"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::T1"))) (kind "feature decl") (name "T1") (declared-name "T1") (parent (node (document "d0") (qualified-name "VehicleUsages"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::T2"))) (kind "feature decl") (name "T2") (declared-name "T2") (parent (node (document "d0") (qualified-name "VehicleUsages"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleUsages"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (kind "feature decl") (name "narrowRimWheel") (declared-name "narrowRimWheel") (parent (node (document "d0") (qualified-name "VehicleUsages"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (kind "feature decl") (name "vehicle_C1") (declared-name "vehicle_C1") (parent (node (document "d0") (qualified-name "VehicleUsages"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (kind "feature decl") (name "vehicle_C2") (declared-name "vehicle_C2") (parent (node (document "d0") (qualified-name "VehicleUsages"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (kind "feature decl") (name "vehicle_C3") (declared-name "vehicle_C3") (parent (node (document "d0") (qualified-name "VehicleUsages"))))
    (element (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (kind "feature decl") (name "wideRimWheel") (declared-name "wideRimWheel") (parent (node (document "d0") (qualified-name "VehicleUsages"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleUsages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleDefinitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
  (document "d0"
    (query (range (start 6 16) (end 6 34)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "VehicleUsages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleDefinitions::*")
        (range (start 6 16) (end 6 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
