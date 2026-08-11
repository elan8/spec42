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
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter"))) (kind "package") (name "SimpleQuadcopter") (declared-name "SimpleQuadcopter") (range (start (line 0) (character 0)) (end (line 0) (character 10751))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 4)) (end (line 1) (character 26))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 19)) (end (line 1) (character 22))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 4)) (end (line 2) (character 25))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 19)) (end (line 2) (character 21))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 4)) (end (line 3) (character 35))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "SpatialItems::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 19)) (end (line 3) (character 31))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 4)) (end (line 4) (character 33))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "ShapeItems::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 19)) (end (line 4) (character 29))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))) (kind "part def") (name "Camera") (declared-name "Camera") (range (start (line 83) (character 4)) (end (line 83) (character 1233))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem") (range (start (line 83) (character 23)) (end (line 83) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::cameraHousing"))) (kind "part") (name "cameraHousing") (declared-name "cameraHousing") (range (start (line 87) (character 8)) (end (line 87) (character 187))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts") (range (start (line 87) (character 30)) (end (line 87) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView"))) (kind "item") (name "fieldOfView") (declared-name "fieldOfView") (range (start (line 97) (character 8)) (end (line 97) (character 504))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::_documentation"))) (kind "documentation") (name "") (range (start (line 97) (character 8)) (end (line 97) (character 504))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView"))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 103) (character 12)) (end (line 103) (character 224))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 103) (character 26)) (end (line 103) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::CoordinateFrame"))) (kind "import") (name "CoordinateFrame") (declared-name "CoordinateFrame") (range (start (line 8) (character 4)) (end (line 8) (character 58))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::CoordinateFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 57))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (kind "part def") (name "PropellerMotorAssy") (declared-name "PropellerMotorAssy") (range (start (line 54) (character 4)) (end (line 54) (character 1158))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem") (range (start (line 54) (character 35)) (end (line 54) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (kind "part") (name "motor") (declared-name "motor") (range (start (line 71) (character 8)) (end (line 71) (character 315))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts") (range (start (line 71) (character 22)) (end (line 71) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 73) (character 12)) (end (line 73) (character 218))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 73) (character 26)) (end (line 73) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (kind "part") (name "propeller") (declared-name "propeller") (range (start (line 58) (character 8)) (end (line 58) (character 479))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts") (range (start (line 58) (character 26)) (end (line 58) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 64) (character 12)) (end (line 64) (character 219))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 64) (character 26)) (end (line 64) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Rotation"))) (kind "import") (name "Rotation") (declared-name "Rotation") (range (start (line 11) (character 4)) (end (line 11) (character 51))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Rotation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 19)) (end (line 11) (character 50))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (kind "part def") (name "Strut") (declared-name "Strut") (range (start (line 20) (character 4)) (end (line 20) (character 1337))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem") (range (start (line 20) (character 22)) (end (line 20) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (kind "part") (name "motorCutout") (declared-name "motorCutout") (range (start (line 39) (character 8)) (end (line 39) (character 322))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts") (range (start (line 39) (character 28)) (end (line 39) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 41) (character 12)) (end (line 41) (character 219))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 41) (character 26)) (end (line 41) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (kind "part") (name "rawStrut") (declared-name "rawStrut") (range (start (line 25) (character 8)) (end (line 25) (character 451))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts") (range (start (line 25) (character 25)) (end (line 25) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 31) (character 12)) (end (line 31) (character 228))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 31) (character 26)) (end (line 31) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::Translation"))) (kind "import") (name "Translation") (declared-name "Translation") (range (start (line 10) (character 4)) (end (line 10) (character 54))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Translation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 53))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::TranslationRotationSequence"))) (kind "import") (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence") (range (start (line 9) (character 4)) (end (line 9) (character 70))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::TranslationRotationSequence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 69))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::motorShape"))) (kind "part") (name "motorShape") (declared-name "motorShape") (range (start (line 13) (character 4)) (end (line 13) (character 155))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpatialItem") (range (start (line 13) (character 22)) (end (line 13) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::pi"))) (kind "import") (name "pi") (declared-name "pi") (range (start (line 6) (character 4)) (end (line 6) (character 37))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "TrigFunctions::pi") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 19)) (end (line 6) (character 36))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (kind "part") (name "quadCopter") (declared-name "quadCopter") (range (start (line 113) (character 4)) (end (line 113) (character 6345))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpatialItem") (range (start (line 113) (character 22)) (end (line 113) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind "part") (name "camera") (declared-name "camera") (range (start (line 236) (character 8)) (end (line 236) (character 364))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Camera") (range (start (line 236) (character 22)) (end (line 236) (character 28)))) (subsetting (reference "subSpatialParts") (range (start (line 236) (character 32)) (end (line 236) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 237) (character 12)) (end (line 237) (character 297))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 237) (character 26)) (end (line 237) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::datum"))) (kind "attribute") (name "datum") (declared-name "datum") (range (start (line 114) (character 8)) (end (line 114) (character 519))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 114) (character 28)) (end (line 114) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (kind "part") (name "mainBody") (declared-name "mainBody") (range (start (line 123) (character 8)) (end (line 123) (character 1750))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts") (range (start (line 123) (character 25)) (end (line 123) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (kind "part") (name "cuttingCornersBox") (declared-name "cuttingCornersBox") (range (start (line 140) (character 12)) (end (line 140) (character 604))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts") (range (start (line 140) (character 38)) (end (line 140) (character 53)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 146) (character 16)) (end (line 146) (character 342))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 146) (character 30)) (end (line 146) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (kind "part") (name "rawBody") (declared-name "rawBody") (range (start (line 126) (character 12)) (end (line 126) (character 498))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts") (range (start (line 126) (character 28)) (end (line 126) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 132) (character 16)) (end (line 132) (character 248))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 132) (character 30)) (end (line 132) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind "part") (name "propellerMotorAssy1") (declared-name "propellerMotorAssy1") (range (start (line 202) (character 8)) (end (line 202) (character 417))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropellerMotorAssy") (range (start (line 202) (character 35)) (end (line 202) (character 53)))) (subsetting (reference "subSpatialParts") (range (start (line 202) (character 57)) (end (line 202) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 203) (character 12)) (end (line 203) (character 324))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 203) (character 26)) (end (line 203) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind "part") (name "propellerMotorAssy2") (declared-name "propellerMotorAssy2") (range (start (line 210) (character 8)) (end (line 210) (character 419))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropellerMotorAssy") (range (start (line 210) (character 35)) (end (line 210) (character 53)))) (subsetting (reference "subSpatialParts") (range (start (line 210) (character 57)) (end (line 210) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 211) (character 12)) (end (line 211) (character 326))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 211) (character 26)) (end (line 211) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind "part") (name "propellerMotorAssy3") (declared-name "propellerMotorAssy3") (range (start (line 218) (character 8)) (end (line 218) (character 420))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropellerMotorAssy") (range (start (line 218) (character 35)) (end (line 218) (character 53)))) (subsetting (reference "subSpatialParts") (range (start (line 218) (character 57)) (end (line 218) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 219) (character 12)) (end (line 219) (character 327))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 219) (character 26)) (end (line 219) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind "part") (name "propellerMotorAssy4") (declared-name "propellerMotorAssy4") (range (start (line 226) (character 8)) (end (line 226) (character 419))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropellerMotorAssy") (range (start (line 226) (character 35)) (end (line 226) (character 53)))) (subsetting (reference "subSpatialParts") (range (start (line 226) (character 57)) (end (line 226) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 227) (character 12)) (end (line 227) (character 326))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 227) (character 26)) (end (line 227) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind "part") (name "strut1") (declared-name "strut1") (range (start (line 169) (character 8)) (end (line 169) (character 390))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Strut") (range (start (line 169) (character 22)) (end (line 169) (character 27)))) (subsetting (reference "subSpatialParts") (range (start (line 169) (character 31)) (end (line 169) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 170) (character 12)) (end (line 170) (character 323))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 170) (character 26)) (end (line 170) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind "part") (name "strut2") (declared-name "strut2") (range (start (line 177) (character 8)) (end (line 177) (character 392))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Strut") (range (start (line 177) (character 22)) (end (line 177) (character 27)))) (subsetting (reference "subSpatialParts") (range (start (line 177) (character 31)) (end (line 177) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 178) (character 12)) (end (line 178) (character 325))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 178) (character 26)) (end (line 178) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind "part") (name "strut3") (declared-name "strut3") (range (start (line 185) (character 8)) (end (line 185) (character 393))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Strut") (range (start (line 185) (character 22)) (end (line 185) (character 27)))) (subsetting (reference "subSpatialParts") (range (start (line 185) (character 31)) (end (line 185) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 186) (character 12)) (end (line 186) (character 326))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 186) (character 26)) (end (line 186) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind "part") (name "strut4") (declared-name "strut4") (range (start (line 193) (character 8)) (end (line 193) (character 392))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature)) (relationships (typing (reference "Strut") (range (start (line 193) (character 22)) (end (line 193) (character 27)))) (subsetting (reference "subSpatialParts") (range (start (line 193) (character 31)) (end (line 193) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 194) (character 12)) (end (line 194) (character 325))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 194) (character 26)) (end (line 194) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (kind "attribute") (name "xStrut") (declared-name "xStrut") (range (start (line 164) (character 8)) (end (line 164) (character 59))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 164) (character 35)) (end (line 164) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (kind "attribute") (name "yStrut") (declared-name "yStrut") (range (start (line 165) (character 8)) (end (line 165) (character 59))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 165) (character 35)) (end (line 165) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (kind "attribute") (name "zPMAssy") (declared-name "zPMAssy") (range (start (line 167) (character 8)) (end (line 167) (character 57))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 167) (character 36)) (end (line 167) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (kind "attribute") (name "zStrut") (declared-name "zStrut") (range (start (line 166) (character 8)) (end (line 166) (character 56))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 166) (character 35)) (end (line 166) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::sqrt"))) (kind "import") (name "sqrt") (declared-name "sqrt") (range (start (line 5) (character 4)) (end (line 5) (character 39))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "RealFunctions::sqrt") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 5) (character 19)) (end (line 5) (character 38))))))
    (element (id (node (document "d0") (qualified-name "SimpleQuadcopter::tan"))) (kind "import") (name "tan") (declared-name "tan") (range (start (line 7) (character 4)) (end (line 7) (character 38))) (parent (node (document "d0") (qualified-name "SimpleQuadcopter"))) (authored (membership (kind Import) (visibility "private") (import (reference "TrigFunctions::tan") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 37))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 19)) (end (line 1) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 2) (character 19)) (end (line 2) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SpatialItems::*") (range (start (line 3) (character 19)) (end (line 3) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ShapeItems::*") (range (start (line 4) (character 19)) (end (line 4) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 83) (character 23)) (end (line 83) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::cameraHousing"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 87) (character 30)) (end (line 87) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 103) (character 26)) (end (line 103) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::CoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::CoordinateFrame") (range (start (line 8) (character 19)) (end (line 8) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 54) (character 35)) (end (line 54) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 71) (character 22)) (end (line 71) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 73) (character 26)) (end (line 73) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 58) (character 26)) (end (line 58) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 64) (character 26)) (end (line 64) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Rotation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Rotation") (range (start (line 11) (character 19)) (end (line 11) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 20) (character 22)) (end (line 20) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 39) (character 28)) (end (line 39) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 41) (character 26)) (end (line 41) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 25) (character 25)) (end (line 25) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 31) (character 26)) (end (line 31) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::Translation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Translation") (range (start (line 10) (character 19)) (end (line 10) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::TranslationRotationSequence"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::TranslationRotationSequence") (range (start (line 9) (character 19)) (end (line 9) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::motorShape"))) (kind featureTyping) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 13) (character 22)) (end (line 13) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::pi"))) (kind membershipImport) (ordinal 0)) (authored-target "TrigFunctions::pi") (range (start (line 6) (character 19)) (end (line 6) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (kind featureTyping) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 113) (character 22)) (end (line 113) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind featureTyping) (ordinal 0)) (authored-target "Camera") (range (start (line 236) (character 22)) (end (line 236) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Camera")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 236) (character 32)) (end (line 236) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 237) (character 26)) (end (line 237) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::datum"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 114) (character 28)) (end (line 114) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 123) (character 25)) (end (line 123) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 140) (character 38)) (end (line 140) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 146) (character 30)) (end (line 146) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 126) (character 28)) (end (line 126) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 132) (character 30)) (end (line 132) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind featureTyping) (ordinal 0)) (authored-target "PropellerMotorAssy") (range (start (line 202) (character 35)) (end (line 202) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 202) (character 57)) (end (line 202) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 203) (character 26)) (end (line 203) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind featureTyping) (ordinal 0)) (authored-target "PropellerMotorAssy") (range (start (line 210) (character 35)) (end (line 210) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 210) (character 57)) (end (line 210) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 211) (character 26)) (end (line 211) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind featureTyping) (ordinal 0)) (authored-target "PropellerMotorAssy") (range (start (line 218) (character 35)) (end (line 218) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 218) (character 57)) (end (line 218) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 219) (character 26)) (end (line 219) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind featureTyping) (ordinal 0)) (authored-target "PropellerMotorAssy") (range (start (line 226) (character 35)) (end (line 226) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 226) (character 57)) (end (line 226) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 227) (character 26)) (end (line 227) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind featureTyping) (ordinal 0)) (authored-target "Strut") (range (start (line 169) (character 22)) (end (line 169) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 169) (character 31)) (end (line 169) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 170) (character 26)) (end (line 170) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind featureTyping) (ordinal 0)) (authored-target "Strut") (range (start (line 177) (character 22)) (end (line 177) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 177) (character 31)) (end (line 177) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 178) (character 26)) (end (line 178) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind featureTyping) (ordinal 0)) (authored-target "Strut") (range (start (line 185) (character 22)) (end (line 185) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 185) (character 31)) (end (line 185) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 186) (character 26)) (end (line 186) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind featureTyping) (ordinal 0)) (authored-target "Strut") (range (start (line 193) (character 22)) (end (line 193) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 193) (character 31)) (end (line 193) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 194) (character 26)) (end (line 194) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 164) (character 35)) (end (line 164) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 165) (character 35)) (end (line 165) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 167) (character 36)) (end (line 167) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 166) (character 35)) (end (line 166) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::sqrt"))) (kind membershipImport) (ordinal 0)) (authored-target "RealFunctions::sqrt") (range (start (line 5) (character 19)) (end (line 5) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SimpleQuadcopter::tan"))) (kind membershipImport) (ordinal 0)) (authored-target "TrigFunctions::tan") (range (start (line 7) (character 19)) (end (line 7) (character 37))) (outcome (status unresolved)))
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
