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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
KwFeature,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
KwFeature,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwComposite,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwComposite,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwComposite,Ident,Colon,Ident,OpenCurly,
KwComposite,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,OpenCurly,
KwComposite,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwComposite,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwComposite,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,
KwComposite,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,OpenCurly,
KwComposite,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwComposite,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFeature,Ident,KwSubsets,Ident,OpenCurly,
KwDoc,RegularComment,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwComposite,Ident,KwSubsets,Ident,ColonColon,Ident,ColonColon,Ident,Eq,Ident,ColonColon,Ident,ColonColon,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenCurly,
KwComposite,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwComposite,Ident,KwSubsets,Ident,ColonColon,Ident,ColonColon,Ident,Eq,Ident,ColonColon,Ident,ColonColon,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenCurly,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,ColonColon,Ident,OpenCurly,
KwComposite,Ident,Colon,Ident,Semicolon,
KwComposite,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConnector,Ident,Colon,Ident,KwFrom,
Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,
Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwComposite,Ident,KwSubsets,Ident,ColonColon,Ident,ColonColon,Ident,Eq,Ident,ColonColon,Ident,ColonColon,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwComposite,Ident,KwSubsets,Ident,ColonColon,Ident,ColonColon,Ident,Eq,Ident,ColonColon,Ident,ColonColon,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConnector,Ident,Colon,Ident,KwFrom,
Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,
Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFeature,Ident,KwSubsets,Ident,OpenCurly,
KwDoc,RegularComment,
KwComposite,Ident,Colon,Ident,OpenCurly,
KwOut,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,ColonColon,Ident,OpenCurly,
KwIn,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwConnector,Ident,KwFrom,
Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleUsages'
    (documentation)
    (import_decl private 'VehicleDefinitions::*')
    (comment)
    (feature_def 'T1' value)
    (feature_def 'T2' value)
    (comment)
    (feature_def 'narrowRimWheel' : 'Wheel'
      (documentation)
      (feature_def composite 'lugbolt' : 'Lugbolt' multiplicity))
    (feature_def 'wideRimWheel' : 'Wheel'
      (documentation)
      (feature_def composite 'lugbolt' : 'Lugbolt' multiplicity))
    (feature_def 'vehicle_C1' : 'Vehicle'
      (documentation)
      (feature_def composite 'frontAxleAssembly' : 'AxleAssembly'
        (feature_def composite 'frontWheel' multiplicity :>> 'narrowRimWheel'
          (feature_def composite 'lugbolt' multiplicity :>> 'narrowRimWheel::lugbolt'
            (feature_def 'tighteningTorque' :>> 'Lugbolt::tighteningTorque' value)))
        (feature_def composite 'frontAxle' : 'Axle'))
      (feature_def composite 'rearAxleAssembly' : 'VehicleDefinitions::AxleAssembly'
        (feature_def composite 'rearWheel' multiplicity :>> 'wideRimWheel'
          (feature_def composite 'lugbolt' multiplicity :>> 'wideRimWheel::lugbolt'
            (feature_def 'tighteningTorque' :>> 'Lugbolt::tighteningTorque' value)))
        (feature_def composite 'rearAxle' : 'Axle')))
    (feature_def 'vehicle_C2' :> 'vehicle_C1'
      (documentation)
      (feature_def composite 'frontAxleAssembly' :>> 'vehicle_C1::frontAxleAssembly'
        (feature_def composite 'leftFrontWheel' :> 'vehicle_C1::frontAxleAssembly::frontWheel' value
          (feature_def composite 'hub' : 'VehicleDefinitions::WheelHubIF'))
        (feature_def composite 'rightFrontWheel' :> 'vehicle_C1::frontAxleAssembly::frontWheel' value
          (feature_def 'hub' : 'VehicleDefinitions::WheelHubIF'))
        (feature_def composite 'frontAxle' :>> 'vehicle_C1::frontAxleAssembly::frontAxle'
          (feature_def composite 'leftMountingPoint' : 'AxleMountIF')
          (feature_def composite 'rightMountingPoint' : 'AxleMountIF'))
        (connector_def 'leftFrontMount' : 'Mounting'
          (connector_end)
          (connector_end))
        (connector_def 'rightFrontMount' : 'Mounting'
          (connector_end)
          (connector_end)))
      (feature_def composite 'rearAxleAssembly' :>> 'vehicle_C1::rearAxleAssembly'
        (feature_def composite 'leftRearWheel' :> 'vehicle_C1::rearAxleAssembly::rearWheel' value
          (feature_def 'hub' : 'WheelHubIF'))
        (feature_def composite 'rightRearWheel' :> 'vehicle_C1::rearAxleAssembly::rearWheel' value
          (feature_def 'hub' : 'WheelHubIF'))
        (feature_def composite 'rearAxle' :>> 'vehicle_C1::rearAxleAssembly::rearAxle'
          (feature_def 'leftMountingPoint' : 'AxleMountIF')
          (feature_def 'rightMountingPoint' : 'AxleMountIF'))
        (connector_def 'leftRearMount' : 'Mounting'
          (connector_end)
          (connector_end))
        (connector_def 'rightRearMount' : 'Mounting'
          (connector_end)
          (connector_end))))
    (feature_def 'vehicle_C3' :> 'vehicle_C2'
      (documentation)
      (feature_def composite 'transmission' : 'Transmission'
        (feature_def out 'drive' : 'DriveIF'))
      (feature_def composite 'rearAxleAssembly' :>> 'vehicle_C2::rearAxleAssembly'
        (feature_def composite 'rearAxle' :>> 'vehicle_C2::rearAxleAssembly::rearAxle'
          (feature_def in 'drive' : 'DriveIF')))
      (connector_def 'driveShaft'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package VehicleUsages {
    doc /*
	 * Example usages of elements from the vehicle definitions model.
	 */

    private import VehicleDefinitions::*;

    /* VALUES */

    feature T1 = 10.0;
    feature T2 = 20.0;

    /* PARTS */

    feature narrowRimWheel : Wheel {
        doc /* Narrow-rim wheel configuration with 4 to 5 lugbolts. */
        composite lugbolt: Lugbolt [4..5];
    }

    feature wideRimWheel : Wheel {
        doc /* Wide-rim wheel configuration with 4 to 6 lugbolts. */
        composite lugbolt: Lugbolt [4..6];
    }

    feature vehicle_C1 : Vehicle {
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
                feature hub : VehicleDefinitions::WheelHubIF;
            }

            composite frontAxle redefines vehicle_C1::frontAxleAssembly::frontAxle {
                composite leftMountingPoint: AxleMountIF;
                composite rightMountingPoint: AxleMountIF;
            }

            connector leftFrontMount : Mounting from frontAxle.leftMountingPoint to leftFrontWheel.hub;

            connector rightFrontMount : Mounting from frontAxle.rightMountingPoint to rightFrontWheel.hub;
        }

        composite rearAxleAssembly redefines vehicle_C1::rearAxleAssembly {
            composite leftRearWheel subsets vehicle_C1::rearAxleAssembly::rearWheel = vehicle_C1::rearAxleAssembly::rearWheel#(1) {
                feature hub : WheelHubIF;
            }
            composite rightRearWheel subsets vehicle_C1::rearAxleAssembly::rearWheel = vehicle_C1::rearAxleAssembly::rearWheel#(2) {
                feature hub : WheelHubIF;
            }

            composite rearAxle redefines vehicle_C1::rearAxleAssembly::rearAxle {
                feature leftMountingPoint : AxleMountIF;
                feature rightMountingPoint : AxleMountIF;
            }

            connector leftRearMount : Mounting from rearAxle.leftMountingPoint to leftRearWheel.hub;

            connector rightRearMount : Mounting from rearAxle.rightMountingPoint to rightRearWheel.hub;
        }
    }

    feature vehicle_C3 subsets vehicle_C2 {
        doc /* Further specialized configuration with a connector to a deeply-nested feature. */
        composite transmission: Transmission {
            out feature drive : DriveIF;
        }

        composite rearAxleAssembly redefines vehicle_C2::rearAxleAssembly {
            composite rearAxle redefines vehicle_C2::rearAxleAssembly::rearAxle {
                in feature drive : DriveIF;
            }
        }

        connector driveShaft from transmission.drive to rearAxleAssembly.rearAxle.drive;
    }
}
~~~
# EXPECTED
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Lugbolt'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Lugbolt'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'AxleAssembly'
semantic.unresolved_name 'Lugbolt::tighteningTorque'
semantic.unresolved_name 'Axle'
semantic.unresolved_name 'VehicleDefinitions::AxleAssembly'
semantic.unresolved_name 'Lugbolt::tighteningTorque'
semantic.unresolved_name 'Axle'
semantic.unresolved_name 'VehicleDefinitions::WheelHubIF'
semantic.unresolved_name 'VehicleDefinitions::WheelHubIF'
semantic.unresolved_name 'AxleMountIF'
semantic.unresolved_name 'AxleMountIF'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'WheelHubIF'
semantic.unresolved_name 'WheelHubIF'
semantic.unresolved_name 'AxleMountIF'
semantic.unresolved_name 'AxleMountIF'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Transmission'
semantic.unresolved_name 'DriveIF'
semantic.unresolved_name 'DriveIF'
~~~
# PROBLEMS
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Lugbolt'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'Lugbolt'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'AxleAssembly'
semantic.unresolved_name 'Lugbolt::tighteningTorque'
semantic.unresolved_name 'Axle'
semantic.unresolved_name 'VehicleDefinitions::AxleAssembly'
semantic.unresolved_name 'Lugbolt::tighteningTorque'
semantic.unresolved_name 'Axle'
semantic.unresolved_name 'VehicleDefinitions::WheelHubIF'
semantic.unresolved_name 'VehicleDefinitions::WheelHubIF'
semantic.unresolved_name 'AxleMountIF'
semantic.unresolved_name 'AxleMountIF'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'WheelHubIF'
semantic.unresolved_name 'WheelHubIF'
semantic.unresolved_name 'AxleMountIF'
semantic.unresolved_name 'AxleMountIF'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Mounting'
semantic.unresolved_name 'Transmission'
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
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "VehicleUsages::T1"))) (name "T1") (declared-name "T1"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "VehicleUsages::T2"))) (name "T2") (declared-name "T2"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleUsages::_documentation"))) (name ""))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "VehicleUsages::narrowRimWheel"))) (name "narrowRimWheel") (declared-name "narrowRimWheel"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C1"))) (name "vehicle_C1") (declared-name "vehicle_C1"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C2"))) (name "vehicle_C2") (declared-name "vehicle_C2"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "VehicleUsages::vehicle_C3"))) (name "vehicle_C3") (declared-name "vehicle_C3"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "VehicleUsages::wideRimWheel"))) (name "wideRimWheel") (declared-name "wideRimWheel"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleUsages::_documentation"))) (to (node (document "d0") (qualified-name "VehicleUsages"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
