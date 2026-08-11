# META
~~~ini
description=SysML Example (Geometry): SimpleQuadcopter
type=file
~~~
# SOURCE
~~~sysml
package SimpleQuadcopter {
    private import ISQ::*;
    private import SI::*;
    private import SpatialItems::*;
    private import ShapeItems::*;
    private import RealFunctions::sqrt;
    private import TrigFunctions::pi;
    private import TrigFunctions::tan;
    private import MeasurementReferences::CoordinateFrame;
    private import MeasurementReferences::TranslationRotationSequence;
    private import MeasurementReferences::Translation;
    private import MeasurementReferences::Rotation;

    part motorShape : SpatialItem {
        item :>> shape : Cylinder {
            :>> radius = 18 [mm];
            :>> height = 30 [mm];
        }
    }
    
    part def Strut :> SpatialItem {
        // By default will get same coordinateFrame.mRefs as owning SpatialItem, i.e.:
        // attribute :>> coordinateFrame { :>> mRefs = (mm, mm, mm); }
        
        /* rawStrut is a construction shape: a rectangular beam */
        part rawStrut :> subSpatialParts {
            item :>> shape : Box {
                :>> length = 160 [mm];
                :>> width = 15 [mm];
                :>> height = 8 [mm];
            }
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (0, shape.width/2, 0)[source]));
                }
            }        
        }

        /* motorCutout is a construction shape: a cylinder of the same shape as the  */
        part motorCutout :> subSpatialParts {
            item :>> shape = motorShape.shape;
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (175, 0, -1)[source]));
                }
            }
        }
        
        /* Strut shape is CSG difference of rawStrut minus motorCutout */
        attribute :> differencesOf[1] {
            item :>> elements = (rawStrut, motorCutout);
        }        
    }
    
    part def PropellerMotorAssy :> SpatialItem {
        // By default will get same coordinateFrame.mRefs as owning CompoundSpatialItem, i.e.:
        // attribute :>> coordinateFrame { :>> mRefs = (mm, mm, mm); }

        part propeller :> subSpatialParts {
            item :>> shape : Cylinder {
                doc /* propeller stay-out volume, without propeller shaft */
                :>> radius = 80 [mm];
                :>> height = 6 [mm];
            }
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (175, 0, 31)[source]));
                }
            }
        }

        part motor :> subSpatialParts {
            item :>> shape = motorShape.shape;
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (175, 0, 0)[source]));
                }
            }
        }
        
        // By default the shape of a PropellerMotorAssy is the union of its owned composite items and parts that are SpatialItems.
    }

    part def Camera :> SpatialItem {
        // By default will get same coordinateFrame.mRefs as owning CompoundSpatialItem, i.e.:
        // attribute :>> coordinateFrame { :>> mRefs = (mm, mm, mm); }

        part cameraHousing :> subSpatialParts {
            item :>> shape : Cylinder {
                :>> radius = 15 [mm];
                :>> height = 24 [mm];
            }
        }

        /* The field of view is modeled as an item, since it is not a part of the quadcopter but rather a stay-out volume 
         * that can for example be used to formulate a constraint.
         */
        item fieldOfView :> subSpatialParts {
            doc /* Conical field of view with half-top angle 20 degree */
            item :>> shape : Cone {
                :>> radius = height * tan(20 * pi/180) [mm];
                :>> height = 500 [mm];
            }
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Rotation( (0, 1, 0)[source], 180['°']));
                }
            }
        }
        
        // By default the shape of a Camera is the union of its owned composite items and parts that are SpatialItems.
    }

    part quadCopter : SpatialItem {
        attribute datum :>> coordinateFrame {
            doc /* The datum is the top level coordinate frame of the system-of-interest, i.e., the quadcopter.
                 * By convention its origin is placed at the bottom of the mainBody with the +X axis pointing in the 
                 * forward fligth (velocity) direction and the +Z axis pointing upward. The +Y axis completes the 
                 * right-handed Cartesian coordinate system.
                 */
            :>> mRefs = (mm, mm, mm);
        }

        part mainBody :> subSpatialParts {

            /* rawBody is a construction shape: the enveloping rectangular box */
            part rawBody :> subSpatialParts {
                item :>> shape : Box {
                    :>> length = 160 [mm];
                    :>> width = 15 [mm];
                    :>> height = 8 [mm];
                }
                attribute :>> coordinateFrame {
                    :>> transformation : TranslationRotationSequence {
                        :>> elements = (new Translation( (0, shape.width/2, 0)[source]));
                    }
                }        
            }
            
            /* cuttingBox is a construction shape: the enveloping rectangular box */
            part cuttingCornersBox :> subSpatialParts {
                item :>> shape : Box {
                    :>> length = 105 [mm];
                    :>> width = 105 [mm];
                    :>> height = 60 [mm];
                }
                attribute :>> coordinateFrame {
                    :>> transformation : TranslationRotationSequence {
                        :>> elements = (new Translation( (0, -shape.length/sqrt(2), -10)[source]), 
                                        new Rotation((0, 0, 1)[source], 45['°']));
                    }
                }        
            }

            /* Main body shape is the CSG intersection of rawBody and cuttingCornersBox */
            attribute :> intersectionsOf[1] {
                item :>> elements = (rawBody, cuttingCornersBox);
            }
            // Current syntax is not end-user friendly
            // It will be possible to specify following simple CSG expression:
            // item :>> shape = rawBody & cuttingCornersBox;
        }

        // Helper construction parameters
        private attribute xStrut : LengthValue = 49.60[mm];
        private attribute yStrut : LengthValue = 24.65[mm];
        private attribute zStrut : LengthValue = 25[mm];
        private attribute zPMAssy : LengthValue = 12[mm];

        part strut1 : Strut :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (xStrut.num, yStrut.num, zStrut.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 45['°']));
                }
            }        
        }
        part strut2 : Strut :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (-xStrut.num, yStrut.num, zStrut.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 135['°']));
                }
            }        
        }
        part strut3 : Strut :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (-xStrut.num, -yStrut.num, zStrut.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 225['°']));
                }
            }        
        }
        part strut4 : Strut :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (xStrut.num, -yStrut.num, zStrut.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 315['°']));
                }
            }        
        }

        part propellerMotorAssy1 : PropellerMotorAssy :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (xStrut.num, yStrut.num, zPMAssy.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 45['°']));
                }
            }        
        }
        part propellerMotorAssy2 : PropellerMotorAssy :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (-xStrut.num, yStrut.num, zPMAssy.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 135['°']));
                }
            }        
        }
        part propellerMotorAssy3 : PropellerMotorAssy :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (-xStrut.num, -yStrut.num, zPMAssy.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 225['°']));
                }
            }        
        }
        part propellerMotorAssy4 : PropellerMotorAssy :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (xStrut.num, -yStrut.num, zPMAssy.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 315['°']));
                }
            }        
        }

        /* The camera is placed protruding from the +X face of the main body, rotated about the +Y axis over 50° downwards */
        part camera : Camera :> subSpatialParts{
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (59, 0, 2)[source]), 
                                    new Rotation((0, 1, 0)[source], 50['°']));
                }
            }        
        }
        
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "simple_quadcopter.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 19) (end 4 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 19) (end 5 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 19) (end 11 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 22) (end 13 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 22) (end 20 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 25) (end 25 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 28) (end 39 43))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 49 8) (end 49 119))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 54 35) (end 54 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 26) (end 58 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 71 22) (end 71 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 83 23) (end 83 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 30) (end 87 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 22) (end 113 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 114 28) (end 114 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 123 25) (end 123 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 126 28) (end 126 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 140 38) (end 140 53))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 155 12) (end 155 329))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 164 8) (end 164 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 8) (end 164 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 35) (end 164 46))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 165 8) (end 165 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 8) (end 165 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 35) (end 165 46))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 166 8) (end 166 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 8) (end 166 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 35) (end 166 46))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 167 8) (end 167 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 8) (end 167 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 36) (end 167 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 169 31) (end 169 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 177 31) (end 177 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 185 31) (end 185 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 193 31) (end 193 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 202 57) (end 202 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 210 57) (end 210 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 218 57) (end 218 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 226 57) (end 226 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 236 32) (end 236 47))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package SimpleQuadcopter {
    private import ISQ::*;
    private import SI::*;
    private import SpatialItems::*;
    private import ShapeItems::*;
    private import RealFunctions::sqrt;
    private import TrigFunctions::pi;
    private import TrigFunctions::tan;
    private import MeasurementReferences::CoordinateFrame;
    private import MeasurementReferences::TranslationRotationSequence;
    private import MeasurementReferences::Translation;
    private import MeasurementReferences::Rotation;

    part motorShape : SpatialItem {
        item :>> shape : Cylinder {
            :>> radius = 18 [mm];
            :>> height = 30 [mm];
        }
    }
    
    part def Strut :> SpatialItem {
        // By default will get same coordinateFrame.mRefs as owning SpatialItem, i.e.:
        // attribute :>> coordinateFrame { :>> mRefs = (mm, mm, mm); }
        
        /* rawStrut is a construction shape: a rectangular beam */
        part rawStrut :> subSpatialParts {
            item :>> shape : Box {
                :>> length = 160 [mm];
                :>> width = 15 [mm];
                :>> height = 8 [mm];
            }
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (0, shape.width/2, 0)[source]));
                }
            }        
        }

        /* motorCutout is a construction shape: a cylinder of the same shape as the  */
        part motorCutout :> subSpatialParts {
            item :>> shape = motorShape.shape;
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (175, 0, -1)[source]));
                }
            }
        }
        
        /* Strut shape is CSG difference of rawStrut minus motorCutout */
        attribute :> differencesOf[1] {
            item :>> elements = (rawStrut, motorCutout);
        }        
    }
    
    part def PropellerMotorAssy :> SpatialItem {
        // By default will get same coordinateFrame.mRefs as owning CompoundSpatialItem, i.e.:
        // attribute :>> coordinateFrame { :>> mRefs = (mm, mm, mm); }

        part propeller :> subSpatialParts {
            item :>> shape : Cylinder {
                doc /* propeller stay-out volume, without propeller shaft */
                :>> radius = 80 [mm];
                :>> height = 6 [mm];
            }
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (175, 0, 31)[source]));
                }
            }
        }

        part motor :> subSpatialParts {
            item :>> shape = motorShape.shape;
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (175, 0, 0)[source]));
                }
            }
        }
        
        // By default the shape of a PropellerMotorAssy is the union of its owned composite items and parts that are SpatialItems.
    }

    part def Camera :> SpatialItem {
        // By default will get same coordinateFrame.mRefs as owning CompoundSpatialItem, i.e.:
        // attribute :>> coordinateFrame { :>> mRefs = (mm, mm, mm); }

        part cameraHousing :> subSpatialParts {
            item :>> shape : Cylinder {
                :>> radius = 15 [mm];
                :>> height = 24 [mm];
            }
        }

        /* The field of view is modeled as an item, since it is not a part of the quadcopter but rather a stay-out volume 
         * that can for example be used to formulate a constraint.
         */
        item fieldOfView :> subSpatialParts {
            doc /* Conical field of view with half-top angle 20 degree */
            item :>> shape : Cone {
                :>> radius = height * tan(20 * pi/180) [mm];
                :>> height = 500 [mm];
            }
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Rotation( (0, 1, 0)[source], 180['°']));
                }
            }
        }
        
        // By default the shape of a Camera is the union of its owned composite items and parts that are SpatialItems.
    }

    part quadCopter : SpatialItem {
        attribute datum :>> coordinateFrame {
            doc /* The datum is the top level coordinate frame of the system-of-interest, i.e., the quadcopter.
                 * By convention its origin is placed at the bottom of the mainBody with the +X axis pointing in the 
                 * forward fligth (velocity) direction and the +Z axis pointing upward. The +Y axis completes the 
                 * right-handed Cartesian coordinate system.
                 */
            :>> mRefs = (mm, mm, mm);
        }

        part mainBody :> subSpatialParts {

            /* rawBody is a construction shape: the enveloping rectangular box */
            part rawBody :> subSpatialParts {
                item :>> shape : Box {
                    :>> length = 160 [mm];
                    :>> width = 15 [mm];
                    :>> height = 8 [mm];
                }
                attribute :>> coordinateFrame {
                    :>> transformation : TranslationRotationSequence {
                        :>> elements = (new Translation( (0, shape.width/2, 0)[source]));
                    }
                }        
            }
            
            /* cuttingBox is a construction shape: the enveloping rectangular box */
            part cuttingCornersBox :> subSpatialParts {
                item :>> shape : Box {
                    :>> length = 105 [mm];
                    :>> width = 105 [mm];
                    :>> height = 60 [mm];
                }
                attribute :>> coordinateFrame {
                    :>> transformation : TranslationRotationSequence {
                        :>> elements = (new Translation( (0, -shape.length/sqrt(2), -10)[source]), 
                                        new Rotation((0, 0, 1)[source], 45['°']));
                    }
                }        
            }

            /* Main body shape is the CSG intersection of rawBody and cuttingCornersBox */
            attribute :> intersectionsOf[1] {
                item :>> elements = (rawBody, cuttingCornersBox);
            }
            // Current syntax is not end-user friendly
            // It will be possible to specify following simple CSG expression:
            // item :>> shape = rawBody & cuttingCornersBox;
        }

        // Helper construction parameters
        private attribute xStrut : LengthValue = 49.60[mm];
        private attribute yStrut : LengthValue = 24.65[mm];
        private attribute zStrut : LengthValue = 25[mm];
        private attribute zPMAssy : LengthValue = 12[mm];

        part strut1 : Strut :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (xStrut.num, yStrut.num, zStrut.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 45['°']));
                }
            }        
        }
        part strut2 : Strut :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (-xStrut.num, yStrut.num, zStrut.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 135['°']));
                }
            }        
        }
        part strut3 : Strut :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (-xStrut.num, -yStrut.num, zStrut.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 225['°']));
                }
            }        
        }
        part strut4 : Strut :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (xStrut.num, -yStrut.num, zStrut.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 315['°']));
                }
            }        
        }

        part propellerMotorAssy1 : PropellerMotorAssy :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (xStrut.num, yStrut.num, zPMAssy.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 45['°']));
                }
            }        
        }
        part propellerMotorAssy2 : PropellerMotorAssy :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (-xStrut.num, yStrut.num, zPMAssy.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 135['°']));
                }
            }        
        }
        part propellerMotorAssy3 : PropellerMotorAssy :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (-xStrut.num, -yStrut.num, zPMAssy.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 225['°']));
                }
            }        
        }
        part propellerMotorAssy4 : PropellerMotorAssy :> subSpatialParts {
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (xStrut.num, -yStrut.num, zPMAssy.num)[source]), 
                                    new Rotation((0, 0, 1)[source], 315['°']));
                }
            }        
        }

        /* The camera is placed protruding from the +X face of the main body, rotated about the +Y axis over 50° downwards */
        part camera : Camera :> subSpatialParts{
            attribute :>> coordinateFrame {
                :>> transformation : TranslationRotationSequence {
                    :>> elements = (new Translation( (59, 0, 2)[source]), 
                                    new Rotation((0, 1, 0)[source], 50['°']));
                }
            }        
        }
        
    }
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "3befefce366cff0064f0ca7e7439162410138893976127b399df82b6ca546863") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter"))) (kind "package") (name "SimpleQuadcopter") (declared-name "SimpleQuadcopter"))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "SpatialItems::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ShapeItems::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))) (kind "part def") (name "Camera") (declared-name "Camera") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::cameraHousing"))) (kind "part") (name "cameraHousing") (declared-name "cameraHousing") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView"))) (kind "item") (name "fieldOfView") (declared-name "fieldOfView") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView"))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::CoordinateFrame"))) (kind "import") (name "CoordinateFrame") (declared-name "CoordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::CoordinateFrame") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (kind "part def") (name "PropellerMotorAssy") (declared-name "PropellerMotorAssy") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (kind "part") (name "motor") (declared-name "motor") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (kind "part") (name "propeller") (declared-name "propeller") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Rotation"))) (kind "import") (name "Rotation") (declared-name "Rotation") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Rotation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (kind "part def") (name "Strut") (declared-name "Strut") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (kind "part") (name "motorCutout") (declared-name "motorCutout") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (kind "part") (name "rawStrut") (declared-name "rawStrut") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Translation"))) (kind "import") (name "Translation") (declared-name "Translation") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Translation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::TranslationRotationSequence"))) (kind "import") (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::TranslationRotationSequence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::motorShape"))) (kind "part") (name "motorShape") (declared-name "motorShape") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpatialItem")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::pi"))) (kind "import") (name "pi") (declared-name "pi") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "TrigFunctions::pi") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (kind "part") (name "quadCopter") (declared-name "quadCopter") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpatialItem")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind "part") (name "camera") (declared-name "camera") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Camera")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::datum"))) (kind "attribute") (name "datum") (declared-name "datum") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (kind "part") (name "mainBody") (declared-name "mainBody") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (kind "part") (name "cuttingCornersBox") (declared-name "cuttingCornersBox") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (kind "part") (name "rawBody") (declared-name "rawBody") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind "part") (name "propellerMotorAssy1") (declared-name "propellerMotorAssy1") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropellerMotorAssy")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind "part") (name "propellerMotorAssy2") (declared-name "propellerMotorAssy2") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropellerMotorAssy")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind "part") (name "propellerMotorAssy3") (declared-name "propellerMotorAssy3") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropellerMotorAssy")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind "part") (name "propellerMotorAssy4") (declared-name "propellerMotorAssy4") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropellerMotorAssy")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind "part") (name "strut1") (declared-name "strut1") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Strut")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind "part") (name "strut2") (declared-name "strut2") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Strut")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind "part") (name "strut3") (declared-name "strut3") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Strut")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind "part") (name "strut4") (declared-name "strut4") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Strut")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (kind "attribute") (name "xStrut") (declared-name "xStrut") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (kind "attribute") (name "yStrut") (declared-name "yStrut") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (kind "attribute") (name "zPMAssy") (declared-name "zPMAssy") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (kind "attribute") (name "zStrut") (declared-name "zStrut") (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::sqrt"))) (kind "import") (name "sqrt") (declared-name "sqrt") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "RealFunctions::sqrt") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::tan"))) (kind "import") (name "tan") (declared-name "tan") (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "TrigFunctions::tan") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SpatialItems::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ShapeItems::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::cameraHousing"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::CoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::CoordinateFrame") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Rotation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Rotation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Translation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Translation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::TranslationRotationSequence"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::TranslationRotationSequence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::motorShape"))) (kind featureTyping) (ordinal 0)) (authored-target "SpatialItem") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::pi"))) (kind membershipImport) (ordinal 0)) (authored-target "TrigFunctions::pi") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (kind featureTyping) (ordinal 0)) (authored-target "SpatialItem") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind featureTyping) (ordinal 0)) (authored-target "Camera") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Camera")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::datum"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind featureTyping) (ordinal 0)) (authored-target "PropellerMotorAssy") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind featureTyping) (ordinal 0)) (authored-target "PropellerMotorAssy") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind featureTyping) (ordinal 0)) (authored-target "PropellerMotorAssy") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind featureTyping) (ordinal 0)) (authored-target "PropellerMotorAssy") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind featureTyping) (ordinal 0)) (authored-target "Strut") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind featureTyping) (ordinal 0)) (authored-target "Strut") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind featureTyping) (ordinal 0)) (authored-target "Strut") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind featureTyping) (ordinal 0)) (authored-target "Strut") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::sqrt"))) (kind membershipImport) (ordinal 0)) (authored-target "RealFunctions::sqrt") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::tan"))) (kind membershipImport) (ordinal 0)) (authored-target "TrigFunctions::tan") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame"))) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 19) (end 2 21)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 2 19) (end 2 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 19) (end 1 22)) (probe (position 1 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 1 19) (end 1 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 169 22) (end 169 27)) (probe (position 169 22))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))
        (kind featureTyping) (ordinal 0) (authored-target "Strut")
        (range (start 169 22) (end 169 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::Strut") (range (start 20 4) (end 20 1337)))
        )
      )
    )
    (query (range (start 177 22) (end 177 27)) (probe (position 177 22))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))
        (kind featureTyping) (ordinal 0) (authored-target "Strut")
        (range (start 177 22) (end 177 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::Strut") (range (start 20 4) (end 20 1337)))
        )
      )
    )
    (query (range (start 185 22) (end 185 27)) (probe (position 185 22))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))
        (kind featureTyping) (ordinal 0) (authored-target "Strut")
        (range (start 185 22) (end 185 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::Strut") (range (start 20 4) (end 20 1337)))
        )
      )
    )
    (query (range (start 193 22) (end 193 27)) (probe (position 193 22))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))
        (kind featureTyping) (ordinal 0) (authored-target "Strut")
        (range (start 193 22) (end 193 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::Strut") (range (start 20 4) (end 20 1337)))
        )
      )
    )
    (query (range (start 236 22) (end 236 28)) (probe (position 236 22))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))
        (kind featureTyping) (ordinal 0) (authored-target "Camera")
        (range (start 236 22) (end 236 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::Camera") (range (start 83 4) (end 83 1233)))
        )
      )
    )
    (query (range (start 4 19) (end 4 29)) (probe (position 4 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "ShapeItems::*")
        (range (start 4 19) (end 4 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 22) (end 13 33)) (probe (position 13 22))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::motorShape"))
        (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
        (range (start 13 22) (end 13 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 22) (end 20 33)) (probe (position 20 22))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Strut"))
        (kind specialization) (ordinal 0) (authored-target "SpatialItem")
        (range (start 20 22) (end 20 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 54 35) (end 54 46)) (probe (position 54 35))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))
        (kind specialization) (ordinal 0) (authored-target "SpatialItem")
        (range (start 54 35) (end 54 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 83 23) (end 83 34)) (probe (position 83 23))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Camera"))
        (kind specialization) (ordinal 0) (authored-target "SpatialItem")
        (range (start 83 23) (end 83 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 113 22) (end 113 33)) (probe (position 113 22))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))
        (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
        (range (start 113 22) (end 113 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 164 35) (end 164 46)) (probe (position 164 35))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 164 35) (end 164 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 165 35) (end 165 46)) (probe (position 165 35))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 165 35) (end 165 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 166 35) (end 166 46)) (probe (position 166 35))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 166 35) (end 166 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 167 36) (end 167 47)) (probe (position 167 36))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 167 36) (end 167 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 19) (end 3 31)) (probe (position 3 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "SpatialItems::*")
        (range (start 3 19) (end 3 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 25 25) (end 25 40)) (probe (position 25 25))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 25 25) (end 25 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 26) (end 31 41)) (probe (position 31 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 31 26) (end 31 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame") (range (start 31 12) (end 31 228)))
        )
      )
    )
    (query (range (start 39 28) (end 39 43)) (probe (position 39 28))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 39 28) (end 39 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 41 26) (end 41 41)) (probe (position 41 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 41 26) (end 41 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame") (range (start 41 12) (end 41 219)))
        )
      )
    )
    (query (range (start 58 26) (end 58 41)) (probe (position 58 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 58 26) (end 58 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 64 26) (end 64 41)) (probe (position 64 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 64 26) (end 64 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame") (range (start 64 12) (end 64 219)))
        )
      )
    )
    (query (range (start 71 22) (end 71 37)) (probe (position 71 22))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 71 22) (end 71 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 73 26) (end 73 41)) (probe (position 73 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 73 26) (end 73 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame") (range (start 73 12) (end 73 218)))
        )
      )
    )
    (query (range (start 87 30) (end 87 45)) (probe (position 87 30))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Camera::cameraHousing"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 87 30) (end 87 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 103 26) (end 103 41)) (probe (position 103 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 103 26) (end 103 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame") (range (start 103 12) (end 103 224)))
        )
      )
    )
    (query (range (start 114 28) (end 114 43)) (probe (position 114 28))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::datum"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 114 28) (end 114 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 123 25) (end 123 40)) (probe (position 123 25))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 123 25) (end 123 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 126 28) (end 126 43)) (probe (position 126 28))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 126 28) (end 126 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 132 30) (end 132 45)) (probe (position 132 30))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 132 30) (end 132 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame") (range (start 132 16) (end 132 248)))
        )
      )
    )
    (query (range (start 140 38) (end 140 53)) (probe (position 140 38))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 140 38) (end 140 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 146 30) (end 146 45)) (probe (position 146 30))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 146 30) (end 146 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame") (range (start 146 16) (end 146 342)))
        )
      )
    )
    (query (range (start 169 31) (end 169 46)) (probe (position 169 31))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 169 31) (end 169 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 170 26) (end 170 41)) (probe (position 170 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 170 26) (end 170 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame") (range (start 170 12) (end 170 323)))
        )
      )
    )
    (query (range (start 177 31) (end 177 46)) (probe (position 177 31))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 177 31) (end 177 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 178 26) (end 178 41)) (probe (position 178 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 178 26) (end 178 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame") (range (start 178 12) (end 178 325)))
        )
      )
    )
    (query (range (start 185 31) (end 185 46)) (probe (position 185 31))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 185 31) (end 185 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 186 26) (end 186 41)) (probe (position 186 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 186 26) (end 186 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame") (range (start 186 12) (end 186 326)))
        )
      )
    )
    (query (range (start 193 31) (end 193 46)) (probe (position 193 31))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 193 31) (end 193 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 194 26) (end 194 41)) (probe (position 194 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 194 26) (end 194 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame") (range (start 194 12) (end 194 325)))
        )
      )
    )
    (query (range (start 202 57) (end 202 72)) (probe (position 202 57))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 202 57) (end 202 72))
        (outcome (status unresolved))
      )
    )
    (query (range (start 203 26) (end 203 41)) (probe (position 203 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 203 26) (end 203 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame") (range (start 203 12) (end 203 324)))
        )
      )
    )
    (query (range (start 210 57) (end 210 72)) (probe (position 210 57))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 210 57) (end 210 72))
        (outcome (status unresolved))
      )
    )
    (query (range (start 211 26) (end 211 41)) (probe (position 211 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 211 26) (end 211 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame") (range (start 211 12) (end 211 326)))
        )
      )
    )
    (query (range (start 218 57) (end 218 72)) (probe (position 218 57))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 218 57) (end 218 72))
        (outcome (status unresolved))
      )
    )
    (query (range (start 219 26) (end 219 41)) (probe (position 219 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 219 26) (end 219 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame") (range (start 219 12) (end 219 327)))
        )
      )
    )
    (query (range (start 226 57) (end 226 72)) (probe (position 226 57))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 226 57) (end 226 72))
        (outcome (status unresolved))
      )
    )
    (query (range (start 227 26) (end 227 41)) (probe (position 227 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 227 26) (end 227 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame") (range (start 227 12) (end 227 326)))
        )
      )
    )
    (query (range (start 236 32) (end 236 47)) (probe (position 236 32))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
        (range (start 236 32) (end 236 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 237 26) (end 237 41)) (probe (position 237 26))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame"))
        (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
        (range (start 237 26) (end 237 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame") (range (start 237 12) (end 237 297)))
        )
      )
    )
    (query (range (start 6 19) (end 6 36)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::pi"))
        (kind membershipImport) (ordinal 0) (authored-target "TrigFunctions::pi")
        (range (start 6 19) (end 6 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 19) (end 7 37)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::tan"))
        (kind membershipImport) (ordinal 0) (authored-target "TrigFunctions::tan")
        (range (start 7 19) (end 7 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 202 35) (end 202 53)) (probe (position 202 35))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))
        (kind featureTyping) (ordinal 0) (authored-target "PropellerMotorAssy")
        (range (start 202 35) (end 202 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy") (range (start 54 4) (end 54 1158)))
        )
      )
    )
    (query (range (start 210 35) (end 210 53)) (probe (position 210 35))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))
        (kind featureTyping) (ordinal 0) (authored-target "PropellerMotorAssy")
        (range (start 210 35) (end 210 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy") (range (start 54 4) (end 54 1158)))
        )
      )
    )
    (query (range (start 218 35) (end 218 53)) (probe (position 218 35))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))
        (kind featureTyping) (ordinal 0) (authored-target "PropellerMotorAssy")
        (range (start 218 35) (end 218 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy") (range (start 54 4) (end 54 1158)))
        )
      )
    )
    (query (range (start 226 35) (end 226 53)) (probe (position 226 35))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))
        (kind featureTyping) (ordinal 0) (authored-target "PropellerMotorAssy")
        (range (start 226 35) (end 226 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy") (range (start 54 4) (end 54 1158)))
        )
      )
    )
    (query (range (start 5 19) (end 5 38)) (probe (position 5 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::sqrt"))
        (kind membershipImport) (ordinal 0) (authored-target "RealFunctions::sqrt")
        (range (start 5 19) (end 5 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 19) (end 11 50)) (probe (position 11 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Rotation"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Rotation")
        (range (start 11 19) (end 11 50))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 19) (end 10 53)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::Translation"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Translation")
        (range (start 10 19) (end 10 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 19) (end 8 57)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::CoordinateFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateFrame")
        (range (start 8 19) (end 8 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 69)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "SimpleQuadcopter::TranslationRotationSequence"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::TranslationRotationSequence")
        (range (start 9 19) (end 9 69))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
