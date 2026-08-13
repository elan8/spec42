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
  (document "memory://snapshot/simple_quadcopter.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 19) (end 4 32))
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
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 14 8) (end 17 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 22) (end 20 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 25 25) (end 25 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 26 12) (end 30 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 31 26) (end 31 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 32 20) (end 32 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 37) (end 32 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 33 20) (end 33 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 39 28) (end 39 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 40 12) (end 40 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 41 26) (end 41 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 42 20) (end 42 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 37) (end 42 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 43 20) (end 43 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 49 21) (end 49 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 50 12) (end 50 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 54 35) (end 54 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 58 26) (end 58 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 59 12) (end 63 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 64 26) (end 64 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 65 20) (end 65 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 37) (end 65 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 66 20) (end 66 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 71 22) (end 71 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 72 12) (end 72 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 73 26) (end 73 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 74 20) (end 74 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 74 37) (end 74 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 75 20) (end 75 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 83 23) (end 83 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 87 30) (end 87 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 88 12) (end 91 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 97 8) (end 108 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 22) (end 113 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 114 28) (end 114 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 120 16) (end 120 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 123 25) (end 123 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 126 28) (end 126 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 127 16) (end 131 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 132 30) (end 132 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 133 24) (end 133 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 133 41) (end 133 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 134 24) (end 134 89))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 140 38) (end 140 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 141 16) (end 145 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 146 30) (end 146 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 147 24) (end 147 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 147 41) (end 147 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 148 24) (end 149 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 155 25) (end 155 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 156 16) (end 156 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 35) (end 164 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 35) (end 165 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 35) (end 166 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 36) (end 167 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 169 31) (end 169 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 170 26) (end 170 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 171 20) (end 171 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 171 37) (end 171 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 172 20) (end 173 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 177 31) (end 177 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 178 26) (end 178 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 179 20) (end 179 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 179 37) (end 179 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 180 20) (end 181 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 185 31) (end 185 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 186 26) (end 186 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 187 20) (end 187 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 187 37) (end 187 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 188 20) (end 189 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 193 31) (end 193 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 194 26) (end 194 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 195 20) (end 195 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 195 37) (end 195 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 196 20) (end 197 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 202 57) (end 202 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 203 26) (end 203 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 204 20) (end 204 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 204 37) (end 204 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 205 20) (end 206 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 210 57) (end 210 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 211 26) (end 211 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 212 20) (end 212 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 212 37) (end 212 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 213 20) (end 214 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 218 57) (end 218 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 219 26) (end 219 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 220 20) (end 220 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 220 37) (end 220 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 221 20) (end 222 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 226 57) (end 226 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 227 26) (end 227 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 228 20) (end 228 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 228 37) (end 228 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 229 20) (end 230 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 236 32) (end 236 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 237 26) (end 237 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 238 20) (end 238 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 238 37) (end 238 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 239 20) (end 240 79))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:231871a63367f5192ae003c6f53679660d22e659fd2653ab7daf5ba47af7cae8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SpatialItems") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ShapeItems") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "RealFunctions::sqrt") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "TrigFunctions::pi") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "TrigFunctions::tan") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::CoordinateFrame") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::TranslationRotationSequence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::Translation") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::Rotation") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Camera"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialItem"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Camera::cameraHousing"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialItem"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialItem"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "differencesOf"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::motorShape"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Camera")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::datum"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mRefs"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "intersectionsOf"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PropellerMotorAssy")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PropellerMotorAssy")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PropellerMotorAssy")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PropellerMotorAssy")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Strut")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Strut")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Strut")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Strut")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationRotationSequence")) (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "LengthValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SpatialItems")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ShapeItems")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "RealFunctions::sqrt")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "TrigFunctions::pi")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "TrigFunctions::tan")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::CoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::Translation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::Rotation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Camera"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Camera::cameraHousing"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "differencesOf")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::motorShape"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind featureTyping) (ordinal 0))
      (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Camera")))))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::datum"))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mRefs")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "intersectionsOf")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind featureTyping) (ordinal 0))
      (authored-target "PropellerMotorAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind featureTyping) (ordinal 0))
      (authored-target "PropellerMotorAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind featureTyping) (ordinal 0))
      (authored-target "PropellerMotorAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind featureTyping) (ordinal 0))
      (authored-target "PropellerMotorAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Strut")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Strut")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind featureTyping) (ordinal 0))
      (authored-target "Strut")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind featureTyping) (ordinal 0))
      (authored-target "Strut")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut")))))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialParts")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Camera"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 1 19) (end 1 25)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 2 19) (end 2 24)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 3 19) (end 3 34)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "SpatialItems")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 4 19) (end 4 32)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ShapeItems")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 5 19) (end 5 38)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "RealFunctions::sqrt")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 6 19) (end 6 36)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "TrigFunctions::pi")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 7 19) (end 7 37)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "TrigFunctions::tan")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 8 19) (end 8 57)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::CoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 9 19) (end 9 69)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 10 19) (end 10 53)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Translation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 11 19) (end 11 50)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::Rotation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 83 23) (end 83 34)) (probe (position 83 23))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Camera"))) (kind specialization) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 87 30) (end 87 45)) (probe (position 87 30))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Camera::cameraHousing"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 54 35) (end 54 46)) (probe (position 54 35))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (kind specialization) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 71 22) (end 71 37)) (probe (position 71 22))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 73 26) (end 73 41)) (probe (position 73 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 74 37) (end 74 64)) (probe (position 74 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 74 20) (end 74 34)) (probe (position 74 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 58 26) (end 58 41)) (probe (position 58 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 64 26) (end 64 41)) (probe (position 64 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 65 37) (end 65 64)) (probe (position 65 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 65 20) (end 65 34)) (probe (position 65 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 20 22) (end 20 33)) (probe (position 20 22))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut"))) (kind specialization) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 49 21) (end 49 34)) (probe (position 49 21))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "differencesOf")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 39 28) (end 39 43)) (probe (position 39 28))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 41 26) (end 41 41)) (probe (position 41 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 42 37) (end 42 64)) (probe (position 42 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 42 20) (end 42 34)) (probe (position 42 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 25 25) (end 25 40)) (probe (position 25 25))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 31 26) (end 31 41)) (probe (position 31 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 32 37) (end 32 64)) (probe (position 32 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 32 20) (end 32 34)) (probe (position 32 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 13 22) (end 13 33)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::motorShape"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 113 22) (end 113 33)) (probe (position 113 22))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 236 22) (end 236 28)) (probe (position 236 22))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind featureTyping) (ordinal 0) (authored-target "Camera")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Camera")))))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 236 32) (end 236 47)) (probe (position 236 32))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 237 26) (end 237 41)) (probe (position 237 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 238 37) (end 238 64)) (probe (position 238 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 238 20) (end 238 34)) (probe (position 238 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 114 28) (end 114 43)) (probe (position 114 28))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::datum"))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 120 16) (end 120 21)) (probe (position 120 16))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mRefs")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 123 25) (end 123 40)) (probe (position 123 25))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 155 25) (end 155 40)) (probe (position 155 25))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "intersectionsOf")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 140 38) (end 140 53)) (probe (position 140 38))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 146 30) (end 146 45)) (probe (position 146 30))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 147 41) (end 147 68)) (probe (position 147 41))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 147 24) (end 147 38)) (probe (position 147 24))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 126 28) (end 126 43)) (probe (position 126 28))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 132 30) (end 132 45)) (probe (position 132 30))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 133 41) (end 133 68)) (probe (position 133 41))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 133 24) (end 133 38)) (probe (position 133 24))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 202 35) (end 202 53)) (probe (position 202 35))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind featureTyping) (ordinal 0) (authored-target "PropellerMotorAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 202 57) (end 202 72)) (probe (position 202 57))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 203 26) (end 203 41)) (probe (position 203 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 204 37) (end 204 64)) (probe (position 204 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 204 20) (end 204 34)) (probe (position 204 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 210 35) (end 210 53)) (probe (position 210 35))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind featureTyping) (ordinal 0) (authored-target "PropellerMotorAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 210 57) (end 210 72)) (probe (position 210 57))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 211 26) (end 211 41)) (probe (position 211 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 212 37) (end 212 64)) (probe (position 212 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 212 20) (end 212 34)) (probe (position 212 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 218 35) (end 218 53)) (probe (position 218 35))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind featureTyping) (ordinal 0) (authored-target "PropellerMotorAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 218 57) (end 218 72)) (probe (position 218 57))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 219 26) (end 219 41)) (probe (position 219 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 220 37) (end 220 64)) (probe (position 220 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 220 20) (end 220 34)) (probe (position 220 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 226 35) (end 226 53)) (probe (position 226 35))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind featureTyping) (ordinal 0) (authored-target "PropellerMotorAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 226 57) (end 226 72)) (probe (position 226 57))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 227 26) (end 227 41)) (probe (position 227 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 228 37) (end 228 64)) (probe (position 228 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 228 20) (end 228 34)) (probe (position 228 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 169 22) (end 169 27)) (probe (position 169 22))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind featureTyping) (ordinal 0) (authored-target "Strut")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut")))))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 169 31) (end 169 46)) (probe (position 169 31))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 170 26) (end 170 41)) (probe (position 170 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 171 37) (end 171 64)) (probe (position 171 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 171 20) (end 171 34)) (probe (position 171 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 177 22) (end 177 27)) (probe (position 177 22))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind featureTyping) (ordinal 0) (authored-target "Strut")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut")))))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 177 31) (end 177 46)) (probe (position 177 31))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 178 26) (end 178 41)) (probe (position 178 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 179 37) (end 179 64)) (probe (position 179 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 179 20) (end 179 34)) (probe (position 179 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 185 22) (end 185 27)) (probe (position 185 22))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind featureTyping) (ordinal 0) (authored-target "Strut")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut")))))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 185 31) (end 185 46)) (probe (position 185 31))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 186 26) (end 186 41)) (probe (position 186 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 187 37) (end 187 64)) (probe (position 187 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 187 20) (end 187 34)) (probe (position 187 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 193 22) (end 193 27)) (probe (position 193 22))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind featureTyping) (ordinal 0) (authored-target "Strut")
      (outcome (status resolved) (target (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::Strut")))))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 193 31) (end 193 46)) (probe (position 193 31))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialParts")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 194 26) (end 194 41)) (probe (position 194 26))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 195 37) (end 195 64)) (probe (position 195 37))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationRotationSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 195 20) (end 195 34)) (probe (position 195 20))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 164 35) (end 164 46)) (probe (position 164 35))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 165 35) (end 165 46)) (probe (position 165 35))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 167 36) (end 167 47)) (probe (position 167 36))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/simple_quadcopter.md") (range (start 166 35) (end 166 46)) (probe (position 166 35))
    (reference (id (source (node (document "memory://snapshot/simple_quadcopter.md") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
)
~~~
