# META
~~~ini
description=SysML Example (Geometry): VehicleGeometryAndCoordinateFrames
type=file
~~~
# SOURCE
~~~sysml
package VehicleGeometryAndCoordinateFrames {
    private import TrigFunctions::*;
    private import ISQ::*;
    private import SI::*;
    private import Time::*;

    private import ShapeItems::*;
    private import SpatialItems::*;

    private import MeasurementReferences::CoordinateFrame;
    private import MeasurementReferences::TranslationRotationSequence;
    private import MeasurementReferences::Translation;
    private import MeasurementReferences::Rotation;
    
    private import Collections::Array;
    private import ScalarValues::Boolean;
    private import ScalarValues::Real;
    private import ScalarValues::Natural;
	private import ControlFunctions::forAll;
    
    part def Vehicle :> SpatialItem;

    part def Chassis :> SpatialItem {
        item :>> shape = new Box(4800 [mm], 1840 [mm], 1350 [mm]);
    }

    part def Wheel :> SpatialItem {
		doc
		/*
		 * Generic wheel with lugbolts
		 * 
		 * The radius is estimated for a 22 inch hub plus 110 mm tire height.
		 * The wheel width is equal to the cylinder height.
		 * The wheel has 5 lugbolts that are evenly distributed along a circle centered at the wheel's center.
		 */
	
        item :>> shape : Cylinder {
            :>> radius = 22/2*25.4 + 110 [mm]; 
            :>> height = 220 [mm];
        }
        attribute <wcf> wheelCoordinateFrame : CoordinateFrame;
        
        attribute numberOfBolts : Natural = 5;	
		part lugBolts : LugBolt[1..numberOfBolts] :> subSpatialParts;
		
		/* 
		 * As an example of a more involved placement of composite parts, constrain the positions of the coordinate frame origins 
		 * of the lugbolts to a circle with radius lbpr distributed evenly over 360°.
		 */
        attribute <lbpr> lugBoltPlacementRadius :>> radius default 60 [mm];
		private attribute lugBoltDistributionAngle :>> planeAngle = 360/numberOfBolts ['°'];
        private attribute lbda : Real = lugBoltDistributionAngle.num * (pi/180); // lugBoltDistributionAngle in radian
		assert constraint {
			(1..numberOfBolts)->forAll {
				in i : Natural;
				private attribute lbcf = lugBolts#(i).coordinateFrame; 
				private attribute trs : TranslationRotationSequence {
					:>> source = wcf;
					:>> target = lbcf;
					:>> elements = new Translation((lbpr*cos((i-1)*lbda), lbpr*sin((i-1)*lbda), -8)[wcf]); 
				}
				lbcf.transformation == trs
			}
		}
    }

    part def LugBolt :> SpatialItem {
    	item :>> shape : Cylinder {
    		:>> radius = 14 [mm];
    		:>> height = 40 [mm];
    	}
    }

    part vehicle : Vehicle, SpatialItem {
		/* 
		 * Vehicle frame origin at center of bottom plate of chassis
		 * with +Z upwards and +X in the forward (front) direction
		 */
        attribute datum :>> coordinateFrame {
            :>> mRefs = (mm, mm, mm);
        }

        part chassis : Chassis[1] :> componentParts {
			attribute :>> coordinateFrame {
				attribute :>> transformation : TranslationRotationSequence {
	 	          	attribute :>> source = datum;
                	attribute :>> elements = new Translation((-(shape as Box).length/2, -(shape as Box).width/2, 0)[datum]);
                }
			}
        }

        private attribute plusXAxis : Array { :>> dimensions = 3; :>> elements : Real[3] = (1, 0, 0); }
        private attribute frontWheelXShift : Real = 1670;
        private attribute rearWheelXShift : Real = -1820;
        private attribute wheelYShift : Real = 720;

        part leftFrontWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
	            	attribute :>> source = datum;
                	attribute :>> elements = (new Translation((frontWheelXShift, wheelYShift, 80)[datum]), new Rotation(plusXAxis[datum], -90['°']));
                }
            }
        }
        part rightFrontWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                	attribute :>> source = datum;
                	attribute :>> elements = (new Translation((frontWheelXShift, -wheelYShift, 80)[datum]), new Rotation((1, 0, 0)[datum], 90['°']));
                }
            }
        }
        part leftRearWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                	attribute :>> source = datum;
                	attribute :>> elements = (new Translation((rearWheelXShift, wheelYShift, 80)[datum]), new Rotation((1, 0, 0)[datum], 90['°']));
                }
            }
        }
        part rightRearWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
					attribute :>> source = datum;
                	attribute :>> elements = (new Translation((rearWheelXShift, -wheelYShift, 80)[datum]), new Rotation((-1, 0, 0)[datum], 90['°']));
                }
            }
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_geometry_and_coordinate_frames.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 19) (end 4 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 19) (end 11 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 19) (end 12 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 19) (end 14 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 19) (end 16 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 19) (end 17 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 24) (end 20 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 24) (end 22 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 22) (end 26 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 8) (end 36 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 8) (end 40 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 47) (end 40 62))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 42 8) (end 42 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 8) (end 42 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 34) (end 42 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 18) (end 43 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 47) (end 43 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 52) (end 49 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 49) (end 50 59))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 51 8) (end 51 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 8) (end 51 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 33) (end 51 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 66 24) (end 66 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 5) (end 67 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 73 28) (end 73 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 78 28) (end 78 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 82 37) (end 82 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 96 42) (end 96 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 104 43) (end 104 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 112 41) (end 112 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 120 42) (end 120 56))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package VehicleGeometryAndCoordinateFrames {
    private import TrigFunctions::*;
    private import ISQ::*;
    private import SI::*;
    private import Time::*;

    private import ShapeItems::*;
    private import SpatialItems::*;

    private import MeasurementReferences::CoordinateFrame;
    private import MeasurementReferences::TranslationRotationSequence;
    private import MeasurementReferences::Translation;
    private import MeasurementReferences::Rotation;
    
    private import Collections::Array;
    private import ScalarValues::Boolean;
    private import ScalarValues::Real;
    private import ScalarValues::Natural;
	private import ControlFunctions::forAll;
    
    part def Vehicle :> SpatialItem;

    part def Chassis :> SpatialItem {
        item :>> shape = new Box(4800 [mm], 1840 [mm], 1350 [mm]);
    }

    part def Wheel :> SpatialItem {
		doc
		/*
		 * Generic wheel with lugbolts
		 * 
		 * The radius is estimated for a 22 inch hub plus 110 mm tire height.
		 * The wheel width is equal to the cylinder height.
		 * The wheel has 5 lugbolts that are evenly distributed along a circle centered at the wheel's center.
		 */
	
        item :>> shape : Cylinder {
            :>> radius = 22/2*25.4 + 110 [mm]; 
            :>> height = 220 [mm];
        }
        attribute <wcf> wheelCoordinateFrame : CoordinateFrame;
        
        attribute numberOfBolts : Natural = 5;	
		part lugBolts : LugBolt[1..numberOfBolts] :> subSpatialParts;
		
		/* 
		 * As an example of a more involved placement of composite parts, constrain the positions of the coordinate frame origins 
		 * of the lugbolts to a circle with radius lbpr distributed evenly over 360°.
		 */
        attribute <lbpr> lugBoltPlacementRadius :>> radius default 60 [mm];
		private attribute lugBoltDistributionAngle :>> planeAngle = 360/numberOfBolts ['°'];
        private attribute lbda : Real = lugBoltDistributionAngle.num * (pi/180); // lugBoltDistributionAngle in radian
		assert constraint {
			(1..numberOfBolts)->forAll {
				in i : Natural;
				private attribute lbcf = lugBolts#(i).coordinateFrame; 
				private attribute trs : TranslationRotationSequence {
					:>> source = wcf;
					:>> target = lbcf;
					:>> elements = new Translation((lbpr*cos((i-1)*lbda), lbpr*sin((i-1)*lbda), -8)[wcf]); 
				}
				lbcf.transformation == trs
			}
		}
    }

    part def LugBolt :> SpatialItem {
    	item :>> shape : Cylinder {
    		:>> radius = 14 [mm];
    		:>> height = 40 [mm];
    	}
    }

    part vehicle : Vehicle, SpatialItem {
		/* 
		 * Vehicle frame origin at center of bottom plate of chassis
		 * with +Z upwards and +X in the forward (front) direction
		 */
        attribute datum :>> coordinateFrame {
            :>> mRefs = (mm, mm, mm);
        }

        part chassis : Chassis[1] :> componentParts {
			attribute :>> coordinateFrame {
				attribute :>> transformation : TranslationRotationSequence {
	 	          	attribute :>> source = datum;
                	attribute :>> elements = new Translation((-(shape as Box).length/2, -(shape as Box).width/2, 0)[datum]);
                }
			}
        }

        private attribute plusXAxis : Array { :>> dimensions = 3; :>> elements : Real[3] = (1, 0, 0); }
        private attribute frontWheelXShift : Real = 1670;
        private attribute rearWheelXShift : Real = -1820;
        private attribute wheelYShift : Real = 720;

        part leftFrontWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
	            	attribute :>> source = datum;
                	attribute :>> elements = (new Translation((frontWheelXShift, wheelYShift, 80)[datum]), new Rotation(plusXAxis[datum], -90['°']));
                }
            }
        }
        part rightFrontWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                	attribute :>> source = datum;
                	attribute :>> elements = (new Translation((frontWheelXShift, -wheelYShift, 80)[datum]), new Rotation((1, 0, 0)[datum], 90['°']));
                }
            }
        }
        part leftRearWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                	attribute :>> source = datum;
                	attribute :>> elements = (new Translation((rearWheelXShift, wheelYShift, 80)[datum]), new Rotation((1, 0, 0)[datum], 90['°']));
                }
            }
        }
        part rightRearWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
					attribute :>> source = datum;
                	attribute :>> elements = (new Translation((rearWheelXShift, -wheelYShift, 80)[datum]), new Rotation((-1, 0, 0)[datum], 90['°']));
                }
            }
        }
    }
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b5b7d6fd4df99f691f0c2c6e6e5dd7a32b38627cc4ca461024b4d4b6955952e3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (kind "package") (name "VehicleGeometryAndCoordinateFrames") (declared-name "VehicleGeometryAndCoordinateFrames") (range (start (line 0) (character 0)) (end (line 0) (character 5063))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 4)) (end (line 1) (character 36))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "TrigFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 19)) (end (line 1) (character 32))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 4)) (end (line 2) (character 26))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 19)) (end (line 2) (character 22))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 4)) (end (line 3) (character 25))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 19)) (end (line 3) (character 21))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 4)) (end (line 4) (character 27))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 19)) (end (line 4) (character 23))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import4"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 4)) (end (line 6) (character 33))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ShapeItems::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 19)) (end (line 6) (character 29))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import5"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 4)) (end (line 7) (character 35))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "SpatialItems::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 31))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Array"))) (kind "import") (name "Array") (declared-name "Array") (range (start (line 14) (character 4)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::Array") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 15) (character 4)) (end (line 15) (character 41))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 40))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis"))) (kind "part def") (name "Chassis") (declared-name "Chassis") (range (start (line 22) (character 4)) (end (line 22) (character 110))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem") (range (start (line 22) (character 24)) (end (line 22) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape"))) (kind "item") (name "shape") (range (start (line 23) (character 8)) (end (line 23) (character 66))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shape") (range (start (line 23) (character 17)) (end (line 23) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::CoordinateFrame"))) (kind "import") (name "CoordinateFrame") (declared-name "CoordinateFrame") (range (start (line 9) (character 4)) (end (line 9) (character 58))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::CoordinateFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 57))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt"))) (kind "part def") (name "LugBolt") (declared-name "LugBolt") (range (start (line 66) (character 4)) (end (line 66) (character 139))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem") (range (start (line 66) (character 24)) (end (line 66) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))) (kind "item") (name "shape") (range (start (line 67) (character 5)) (end (line 67) (character 95))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range none)) (redefinition (reference "shape") (range (start (line 67) (character 14)) (end (line 67) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 69) (character 6)) (end (line 69) (character 27))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height") (range (start (line 69) (character 6)) (end (line 69) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 68) (character 6)) (end (line 68) (character 27))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 68) (character 6)) (end (line 68) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 17) (character 4)) (end (line 17) (character 41))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 40))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 16) (character 4)) (end (line 16) (character 38))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Rotation"))) (kind "import") (name "Rotation") (declared-name "Rotation") (range (start (line 12) (character 4)) (end (line 12) (character 51))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Rotation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 19)) (end (line 12) (character 50))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Translation"))) (kind "import") (name "Translation") (declared-name "Translation") (range (start (line 11) (character 4)) (end (line 11) (character 54))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Translation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 19)) (end (line 11) (character 53))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::TranslationRotationSequence"))) (kind "import") (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence") (range (start (line 10) (character 4)) (end (line 10) (character 70))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::TranslationRotationSequence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 69))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 20) (character 4)) (end (line 20) (character 36))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem") (range (start (line 20) (character 24)) (end (line 20) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 26) (character 4)) (end (line 26) (character 1526))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem") (range (start (line 26) (character 22)) (end (line 26) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::_documentation"))) (kind "documentation") (name "") (range (start (line 26) (character 4)) (end (line 26) (character 1526))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lbda"))) (kind "attribute") (name "lbda") (declared-name "lbda") (range (start (line 51) (character 8)) (end (line 51) (character 80))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 51) (character 33)) (end (line 51) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltDistributionAngle"))) (kind "attribute") (name "lugBoltDistributionAngle") (declared-name "lugBoltDistributionAngle") (range (start (line 50) (character 2)) (end (line 50) (character 87))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (authored (membership (kind Feature) (visibility "private")) (relationships (redefinition (reference "planeAngle") (range (start (line 50) (character 49)) (end (line 50) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltPlacementRadius"))) (kind "attribute") (name "lugBoltPlacementRadius") (declared-name "lugBoltPlacementRadius") (range (start (line 49) (character 8)) (end (line 49) (character 75))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 49) (character 52)) (end (line 49) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBolts"))) (kind "part") (name "lugBolts") (declared-name "lugBolts") (range (start (line 43) (character 2)) (end (line 43) (character 63))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBolt") (range (start (line 43) (character 18)) (end (line 43) (character 25)))) (subsetting (reference "subSpatialParts") (range (start (line 43) (character 47)) (end (line 43) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::numberOfBolts"))) (kind "attribute") (name "numberOfBolts") (declared-name "numberOfBolts") (range (start (line 42) (character 8)) (end (line 42) (character 46))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural") (range none)) (typing (reference "Natural") (range (start (line 42) (character 34)) (end (line 42) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))) (kind "item") (name "shape") (range (start (line 36) (character 8)) (end (line 36) (character 128))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range none)) (redefinition (reference "shape") (range (start (line 36) (character 17)) (end (line 36) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 38) (character 12)) (end (line 38) (character 34))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height") (range (start (line 38) (character 12)) (end (line 38) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 37) (character 12)) (end (line 37) (character 46))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 37) (character 12)) (end (line 37) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::wheelCoordinateFrame"))) (kind "attribute") (name "wheelCoordinateFrame") (declared-name "wheelCoordinateFrame") (range (start (line 40) (character 8)) (end (line 40) (character 63))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "CoordinateFrame") (range none)) (typing (reference "CoordinateFrame") (range (start (line 40) (character 47)) (end (line 40) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 18) (character 1)) (end (line 18) (character 41))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 16)) (end (line 18) (character 40))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 73) (character 4)) (end (line 73) (character 2556))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 73) (character 19)) (end (line 73) (character 26)))) (typing (reference "SpatialItem") (range (start (line 73) (character 28)) (end (line 73) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))) (kind "part") (name "chassis") (declared-name "chassis") (range (start (line 82) (character 8)) (end (line 82) (character 352))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Chassis") (range (start (line 82) (character 23)) (end (line 82) (character 30)))) (subsetting (reference "componentParts") (range (start (line 82) (character 37)) (end (line 82) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 83) (character 3)) (end (line 83) (character 288))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 83) (character 17)) (end (line 83) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::datum"))) (kind "attribute") (name "datum") (declared-name "datum") (range (start (line 78) (character 8)) (end (line 78) (character 93))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 78) (character 28)) (end (line 78) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))) (kind "attribute") (name "frontWheelXShift") (declared-name "frontWheelXShift") (range (start (line 92) (character 8)) (end (line 92) (character 57))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 92) (character 45)) (end (line 92) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))) (kind "part") (name "leftFrontWheel") (declared-name "leftFrontWheel") (range (start (line 96) (character 8)) (end (line 96) (character 413))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 96) (character 30)) (end (line 96) (character 35)))) (subsetting (reference "componentParts") (range (start (line 96) (character 42)) (end (line 96) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 97) (character 12)) (end (line 97) (character 344))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 97) (character 26)) (end (line 97) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))) (kind "part") (name "leftRearWheel") (declared-name "leftRearWheel") (range (start (line 112) (character 8)) (end (line 112) (character 413))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 112) (character 29)) (end (line 112) (character 34)))) (subsetting (reference "componentParts") (range (start (line 112) (character 41)) (end (line 112) (character 55)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 113) (character 12)) (end (line 113) (character 345))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 113) (character 26)) (end (line 113) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::plusXAxis"))) (kind "attribute") (name "plusXAxis") (declared-name "plusXAxis") (range (start (line 91) (character 8)) (end (line 91) (character 103))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "Array") (range none)) (typing (reference "Array") (range (start (line 91) (character 38)) (end (line 91) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))) (kind "attribute") (name "rearWheelXShift") (declared-name "rearWheelXShift") (range (start (line 93) (character 8)) (end (line 93) (character 57))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 93) (character 44)) (end (line 93) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))) (kind "part") (name "rightFrontWheel") (declared-name "rightFrontWheel") (range (start (line 104) (character 8)) (end (line 104) (character 417))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 104) (character 31)) (end (line 104) (character 36)))) (subsetting (reference "componentParts") (range (start (line 104) (character 43)) (end (line 104) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 105) (character 12)) (end (line 105) (character 347))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 105) (character 26)) (end (line 105) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))) (kind "part") (name "rightRearWheel") (declared-name "rightRearWheel") (range (start (line 120) (character 8)) (end (line 120) (character 404))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 120) (character 30)) (end (line 120) (character 35)))) (subsetting (reference "componentParts") (range (start (line 120) (character 42)) (end (line 120) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 121) (character 12)) (end (line 121) (character 335))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 121) (character 26)) (end (line 121) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))) (kind "attribute") (name "wheelYShift") (declared-name "wheelYShift") (range (start (line 94) (character 8)) (end (line 94) (character 51))) (parent (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 94) (character 40)) (end (line 94) (character 44)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "TrigFunctions::*") (range (start (line 1) (character 19)) (end (line 1) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 19)) (end (line 2) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 3) (character 19)) (end (line 3) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "Time::*") (range (start (line 4) (character 19)) (end (line 4) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import4"))) (kind namespaceImport) (ordinal 0)) (authored-target "ShapeItems::*") (range (start (line 6) (character 19)) (end (line 6) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import5"))) (kind namespaceImport) (ordinal 0)) (authored-target "SpatialItems::*") (range (start (line 7) (character 19)) (end (line 7) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Array"))) (kind membershipImport) (ordinal 0)) (authored-target "Collections::Array") (range (start (line 14) (character 19)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 15) (character 19)) (end (line 15) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 22) (character 24)) (end (line 22) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape"))) (kind redefinition) (ordinal 0)) (authored-target "shape") (range (start (line 23) (character 17)) (end (line 23) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::CoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::CoordinateFrame") (range (start (line 9) (character 19)) (end (line 9) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 66) (character 24)) (end (line 66) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))) (kind redefinition) (ordinal 0)) (authored-target "shape") (range (start (line 67) (character 14)) (end (line 67) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (range (start (line 69) (character 6)) (end (line 69) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::height")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 68) (character 6)) (end (line 68) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 17) (character 19)) (end (line 17) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 16) (character 19)) (end (line 16) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Rotation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Rotation") (range (start (line 12) (character 19)) (end (line 12) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Translation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Translation") (range (start (line 11) (character 19)) (end (line 11) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::TranslationRotationSequence"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::TranslationRotationSequence") (range (start (line 10) (character 19)) (end (line 10) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 20) (character 24)) (end (line 20) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 26) (character 22)) (end (line 26) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lbda"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lbda"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 51) (character 33)) (end (line 51) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltDistributionAngle"))) (kind redefinition) (ordinal 0)) (authored-target "planeAngle") (range (start (line 50) (character 49)) (end (line 50) (character 59))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltPlacementRadius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 49) (character 52)) (end (line 49) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBolts"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBolt") (range (start (line 43) (character 18)) (end (line 43) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBolts"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 43) (character 47)) (end (line 43) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::numberOfBolts"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::numberOfBolts"))) (kind featureTyping) (ordinal 1)) (authored-target "Natural") (range (start (line 42) (character 34)) (end (line 42) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))) (kind redefinition) (ordinal 0)) (authored-target "shape") (range (start (line 36) (character 17)) (end (line 36) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (range (start (line 38) (character 12)) (end (line 38) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::height")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 37) (character 12)) (end (line 37) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::wheelCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::wheelCoordinateFrame"))) (kind featureTyping) (ordinal 1)) (authored-target "CoordinateFrame") (range (start (line 40) (character 47)) (end (line 40) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 18) (character 16)) (end (line 18) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 73) (character 19)) (end (line 73) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (kind featureTyping) (ordinal 1)) (authored-target "SpatialItem") (range (start (line 73) (character 28)) (end (line 73) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))) (kind featureTyping) (ordinal 0)) (authored-target "Chassis") (range (start (line 82) (character 23)) (end (line 82) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))) (kind subsetting) (ordinal 0)) (authored-target "componentParts") (range (start (line 82) (character 37)) (end (line 82) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 83) (character 17)) (end (line 83) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::datum"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 78) (character 28)) (end (line 78) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 92) (character 45)) (end (line 92) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 96) (character 30)) (end (line 96) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))) (kind subsetting) (ordinal 0)) (authored-target "componentParts") (range (start (line 96) (character 42)) (end (line 96) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 97) (character 26)) (end (line 97) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 112) (character 29)) (end (line 112) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))) (kind subsetting) (ordinal 0)) (authored-target "componentParts") (range (start (line 112) (character 41)) (end (line 112) (character 55))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 113) (character 26)) (end (line 113) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::plusXAxis"))) (kind featureTyping) (ordinal 0)) (authored-target "Array") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Array")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::plusXAxis"))) (kind featureTyping) (ordinal 1)) (authored-target "Array") (range (start (line 91) (character 38)) (end (line 91) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Array")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 93) (character 44)) (end (line 93) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 104) (character 31)) (end (line 104) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))) (kind subsetting) (ordinal 0)) (authored-target "componentParts") (range (start (line 104) (character 43)) (end (line 104) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 105) (character 26)) (end (line 105) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 120) (character 30)) (end (line 120) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))) (kind subsetting) (ordinal 0)) (authored-target "componentParts") (range (start (line 120) (character 42)) (end (line 120) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 121) (character 26)) (end (line 121) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 94) (character 40)) (end (line 94) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::height"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::height"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::height"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::radius"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::height"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::height"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::height"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::radius"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis::coordinateFrame"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel::coordinateFrame"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel::coordinateFrame"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::plusXAxis"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Array"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::plusXAxis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::plusXAxis"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Array"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::plusXAxis"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel::coordinateFrame"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel::coordinateFrame"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))) (target (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))) (kind featureTyping) (ordinal 1)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lbda")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltDistributionAngle")) (expression (status "ok") (value (integer 72))))
    (node (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltPlacementRadius")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::numberOfBolts")) (expression (status "ok") (value (integer 5))))
    (node (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift")) (expression (status "ok") (value (integer 1670))))
    (node (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift")) (expression (status "ok") (value (integer -1820))))
    (node (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift")) (expression (status "ok") (value (integer 720))))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 19) (end 3 21)) (probe (position 3 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 3 19) (end 3 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 19) (end 2 22)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 19) (end 2 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 19) (end 4 23)) (probe (position 4 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "Time::*")
        (range (start 4 19) (end 4 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 51 33) (end 51 37)) (probe (position 51 33))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lbda"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 51 33) (end 51 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 92 45) (end 92 49)) (probe (position 92 45))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 92 45) (end 92 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real") (range (start 16 4) (end 16 38)))
        )
      )
    )
    (query (range (start 93 44) (end 93 48)) (probe (position 93 44))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 93 44) (end 93 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real") (range (start 16 4) (end 16 38)))
        )
      )
    )
    (query (range (start 94 40) (end 94 44)) (probe (position 94 40))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 94 40) (end 94 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real") (range (start 16 4) (end 16 38)))
        )
      )
    )
    (query (range (start 23 17) (end 23 22)) (probe (position 23 17))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape"))
        (kind redefinition) (ordinal 0) (authored-target "shape")
        (range (start 23 17) (end 23 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape") (range (start 23 8) (end 23 66)))
        )
      )
    )
    (query (range (start 36 17) (end 36 22)) (probe (position 36 17))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))
        (kind redefinition) (ordinal 0) (authored-target "shape")
        (range (start 36 17) (end 36 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape") (range (start 36 8) (end 36 128)))
        )
      )
    )
    (query (range (start 67 14) (end 67 19)) (probe (position 67 14))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))
        (kind redefinition) (ordinal 0) (authored-target "shape")
        (range (start 67 14) (end 67 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape") (range (start 67 5) (end 67 95)))
        )
      )
    )
    (query (range (start 91 38) (end 91 43)) (probe (position 91 38))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::plusXAxis"))
        (kind featureTyping) (ordinal 1) (authored-target "Array")
        (range (start 91 38) (end 91 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Array") (range (start 14 4) (end 14 38)))
        )
      )
    )
    (query (range (start 96 30) (end 96 35)) (probe (position 96 30))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 96 30) (end 96 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel") (range (start 26 4) (end 26 1526)))
        )
      )
    )
    (query (range (start 104 31) (end 104 36)) (probe (position 104 31))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 104 31) (end 104 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel") (range (start 26 4) (end 26 1526)))
        )
      )
    )
    (query (range (start 112 29) (end 112 34)) (probe (position 112 29))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 112 29) (end 112 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel") (range (start 26 4) (end 26 1526)))
        )
      )
    )
    (query (range (start 120 30) (end 120 35)) (probe (position 120 30))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 120 30) (end 120 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel") (range (start 26 4) (end 26 1526)))
        )
      )
    )
    (query (range (start 49 52) (end 49 58)) (probe (position 49 52))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltPlacementRadius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 49 52) (end 49 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 42 34) (end 42 41)) (probe (position 42 34))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::numberOfBolts"))
        (kind featureTyping) (ordinal 1) (authored-target "Natural")
        (range (start 42 34) (end 42 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 43 18) (end 43 25)) (probe (position 43 18))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBolts"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBolt")
        (range (start 43 18) (end 43 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 73 19) (end 73 26)) (probe (position 73 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 73 19) (end 73 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle") (range (start 20 4) (end 20 36)))
        )
      )
    )
    (query (range (start 82 23) (end 82 30)) (probe (position 82 23))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))
        (kind featureTyping) (ordinal 0) (authored-target "Chassis")
        (range (start 82 23) (end 82 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis") (range (start 22 4) (end 22 110)))
        )
      )
    )
    (query (range (start 6 19) (end 6 29)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import4"))
        (kind namespaceImport) (ordinal 0) (authored-target "ShapeItems::*")
        (range (start 6 19) (end 6 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 37 12) (end 37 22)) (probe (position 37 12))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 37 12) (end 37 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::radius") (range (start 37 12) (end 37 46)))
        )
      )
    )
    (query (range (start 38 12) (end 38 22)) (probe (position 38 12))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::height"))
        (kind redefinition) (ordinal 0) (authored-target "height")
        (range (start 38 12) (end 38 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::height") (range (start 38 12) (end 38 34)))
        )
      )
    )
    (query (range (start 50 49) (end 50 59)) (probe (position 50 49))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltDistributionAngle"))
        (kind redefinition) (ordinal 0) (authored-target "planeAngle")
        (range (start 50 49) (end 50 59))
        (outcome (status unresolved))
      )
    )
    (query (range (start 68 6) (end 68 16)) (probe (position 68 6))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 68 6) (end 68 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::radius") (range (start 68 6) (end 68 27)))
        )
      )
    )
    (query (range (start 69 6) (end 69 16)) (probe (position 69 6))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::height"))
        (kind redefinition) (ordinal 0) (authored-target "height")
        (range (start 69 6) (end 69 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::height") (range (start 69 6) (end 69 27)))
        )
      )
    )
    (query (range (start 20 24) (end 20 35)) (probe (position 20 24))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))
        (kind specialization) (ordinal 0) (authored-target "SpatialItem")
        (range (start 20 24) (end 20 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 24) (end 22 35)) (probe (position 22 24))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis"))
        (kind specialization) (ordinal 0) (authored-target "SpatialItem")
        (range (start 22 24) (end 22 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 22) (end 26 33)) (probe (position 26 22))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))
        (kind specialization) (ordinal 0) (authored-target "SpatialItem")
        (range (start 26 22) (end 26 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 66 24) (end 66 35)) (probe (position 66 24))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt"))
        (kind specialization) (ordinal 0) (authored-target "SpatialItem")
        (range (start 66 24) (end 66 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 73 28) (end 73 39)) (probe (position 73 28))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))
        (kind featureTyping) (ordinal 1) (authored-target "SpatialItem")
        (range (start 73 28) (end 73 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 19) (end 7 31)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import5"))
        (kind namespaceImport) (ordinal 0) (authored-target "SpatialItems::*")
        (range (start 7 19) (end 7 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 19) (end 1 32)) (probe (position 1 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "TrigFunctions::*")
        (range (start 1 19) (end 1 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 82 37) (end 82 51)) (probe (position 82 37))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))
        (kind subsetting) (ordinal 0) (authored-target "componentParts")
        (range (start 82 37) (end 82 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 96 42) (end 96 56)) (probe (position 96 42))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))
        (kind subsetting) (ordinal 0) (authored-target "componentParts")
        (range (start 96 42) (end 96 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 104 43) (end 104 57)) (probe (position 104 43))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))
        (kind subsetting) (ordinal 0) (authored-target "componentParts")
        (range (start 104 43) (end 104 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 112 41) (end 112 55)) (probe (position 112 41))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))
        (kind subsetting) (ordinal 0) (authored-target "componentParts")
        (range (start 112 41) (end 112 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 120 42) (end 120 56)) (probe (position 120 42))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))
        (kind subsetting) (ordinal 0) (authored-target "componentParts")
        (range (start 120 42) (end 120 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 47) (end 40 62)) (probe (position 40 47))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::wheelCoordinateFrame"))
        (kind featureTyping) (ordinal 1) (authored-target "CoordinateFrame")
        (range (start 40 47) (end 40 62))
        (outcome (status unresolved))
      )
    )
    (query (range (start 43 47) (end 43 62)) (probe (position 43 47))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBolts"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 43 47) (end 43 62))
        (outcome (status unresolved))
      )
    )
    (query (range (start 78 28) (end 78 43)) (probe (position 78 28))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::datum"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 78 28) (end 78 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 83 17) (end 83 32)) (probe (position 83 17))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 83 17) (end 83 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis::coordinateFrame") (range (start 83 3) (end 83 288)))
        )
      )
    )
    (query (range (start 97 26) (end 97 41)) (probe (position 97 26))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 97 26) (end 97 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel::coordinateFrame") (range (start 97 12) (end 97 344)))
        )
      )
    )
    (query (range (start 105 26) (end 105 41)) (probe (position 105 26))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 105 26) (end 105 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel::coordinateFrame") (range (start 105 12) (end 105 347)))
        )
      )
    )
    (query (range (start 113 26) (end 113 41)) (probe (position 113 26))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 113 26) (end 113 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel::coordinateFrame") (range (start 113 12) (end 113 345)))
        )
      )
    )
    (query (range (start 121 26) (end 121 41)) (probe (position 121 26))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 121 26) (end 121 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel::coordinateFrame") (range (start 121 12) (end 121 335)))
        )
      )
    )
    (query (range (start 14 19) (end 14 37)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Array"))
        (kind membershipImport) (ordinal 0) (authored-target "Collections::Array")
        (range (start 14 19) (end 14 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 19) (end 16 37)) (probe (position 16 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 16 19) (end 16 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 19) (end 15 40)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 15 19) (end 15 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 19) (end 17 40)) (probe (position 17 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 17 19) (end 17 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 40)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 18 16) (end 18 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 19) (end 12 50)) (probe (position 12 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Rotation"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Rotation")
        (range (start 12 19) (end 12 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 19) (end 11 53)) (probe (position 11 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Translation"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Translation")
        (range (start 11 19) (end 11 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 57)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::CoordinateFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateFrame")
        (range (start 9 19) (end 9 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 19) (end 10 69)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::TranslationRotationSequence"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::TranslationRotationSequence")
        (range (start 10 19) (end 10 69))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
