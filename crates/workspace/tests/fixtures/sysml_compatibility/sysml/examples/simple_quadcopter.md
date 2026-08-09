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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
LineComment,
LineComment,
RegularComment,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,Ident,Dot,Ident,Slash,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
RegularComment,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,Minus,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,ColonGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwItem,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
LineComment,
LineComment,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
LineComment,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
LineComment,
LineComment,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
RegularComment,
KwItem,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Star,Ident,OpenParen,DecimalValue,Star,Ident,Slash,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
LineComment,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,RegularComment,
ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
RegularComment,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,Ident,Dot,Ident,Slash,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
RegularComment,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,Minus,Ident,Dot,Ident,Slash,Ident,OpenParen,DecimalValue,CloseParen,Comma,Minus,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,ColonGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwItem,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
LineComment,
LineComment,
LineComment,
CloseCurly,
LineComment,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Minus,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Minus,Ident,Dot,Ident,Comma,Minus,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Dot,Ident,Comma,Minus,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Minus,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Minus,Ident,Dot,Ident,Comma,Minus,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Dot,Ident,Comma,Minus,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
RegularComment,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,
Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'SimpleQuadcopter'
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private 'SpatialItems::*')
    (import_decl private 'ShapeItems::*')
    (import_decl private 'RealFunctions::sqrt')
    (import_decl private 'TrigFunctions::pi')
    (import_decl private 'TrigFunctions::tan')
    (import_decl private 'MeasurementReferences::CoordinateFrame')
    (import_decl private 'MeasurementReferences::TranslationRotationSequence')
    (import_decl private 'MeasurementReferences::Translation')
    (import_decl private 'MeasurementReferences::Rotation')
    (part_usage 'motorShape' : 'SpatialItem'
      (item_usage :>> 'shape' : 'Cylinder'
        (default_ref_usage :>> 'radius' value)
        (default_ref_usage :>> 'height' value)))
    (part_def 'Strut' :> 'SpatialItem'
      (line_comment)
      (line_comment)
      (comment)
      (part_usage 'rawStrut' :> 'subSpatialParts'
        (item_usage :>> 'shape' : 'Box'
          (default_ref_usage :>> 'length' value)
          (default_ref_usage :>> 'width' value)
          (default_ref_usage :>> 'height' value))
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (comment)
      (part_usage 'motorCutout' :> 'subSpatialParts'
        (item_usage :>> 'shape' value)
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (comment)
      (attribute_usage :> 'differencesOf' multiplicity
        (item_usage :>> 'elements' value)))
    (part_def 'PropellerMotorAssy' :> 'SpatialItem'
      (line_comment)
      (line_comment)
      (part_usage 'propeller' :> 'subSpatialParts'
        (item_usage :>> 'shape' : 'Cylinder'
          (documentation)
          (default_ref_usage :>> 'radius' value)
          (default_ref_usage :>> 'height' value))
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (part_usage 'motor' :> 'subSpatialParts'
        (item_usage :>> 'shape' value)
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (line_comment))
    (part_def 'Camera' :> 'SpatialItem'
      (line_comment)
      (line_comment)
      (part_usage 'cameraHousing' :> 'subSpatialParts'
        (item_usage :>> 'shape' : 'Cylinder'
          (default_ref_usage :>> 'radius' value)
          (default_ref_usage :>> 'height' value)))
      (comment)
      (item_usage 'fieldOfView' :> 'subSpatialParts'
        (documentation)
        (item_usage :>> 'shape' : 'Cone'
          (default_ref_usage :>> 'radius' value)
          (default_ref_usage :>> 'height' value))
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (line_comment))
    (part_usage 'quadCopter' : 'SpatialItem'
      (attribute_usage 'datum' :>> 'coordinateFrame'
        (documentation)
        (default_ref_usage :>> 'mRefs' value))
      (part_usage 'mainBody' :> 'subSpatialParts'
        (comment)
        (part_usage 'rawBody' :> 'subSpatialParts'
          (item_usage :>> 'shape' : 'Box'
            (default_ref_usage :>> 'length' value)
            (default_ref_usage :>> 'width' value)
            (default_ref_usage :>> 'height' value))
          (attribute_usage :>> 'coordinateFrame'
            (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
              (default_ref_usage :>> 'elements' value))))
        (comment)
        (part_usage 'cuttingCornersBox' :> 'subSpatialParts'
          (item_usage :>> 'shape' : 'Box'
            (default_ref_usage :>> 'length' value)
            (default_ref_usage :>> 'width' value)
            (default_ref_usage :>> 'height' value))
          (attribute_usage :>> 'coordinateFrame'
            (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
              (default_ref_usage :>> 'elements' value))))
        (comment)
        (attribute_usage :> 'intersectionsOf' multiplicity
          (item_usage :>> 'elements' value))
        (line_comment)
        (line_comment)
        (line_comment))
      (line_comment)
      (attribute_usage private 'xStrut' : 'LengthValue' value)
      (attribute_usage private 'yStrut' : 'LengthValue' value)
      (attribute_usage private 'zStrut' : 'LengthValue' value)
      (attribute_usage private 'zPMAssy' : 'LengthValue' value)
      (part_usage 'strut1' : 'Strut' :> 'subSpatialParts'
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (part_usage 'strut2' : 'Strut' :> 'subSpatialParts'
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (part_usage 'strut3' : 'Strut' :> 'subSpatialParts'
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (part_usage 'strut4' : 'Strut' :> 'subSpatialParts'
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (part_usage 'propellerMotorAssy1' : 'PropellerMotorAssy' :> 'subSpatialParts'
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (part_usage 'propellerMotorAssy2' : 'PropellerMotorAssy' :> 'subSpatialParts'
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (part_usage 'propellerMotorAssy3' : 'PropellerMotorAssy' :> 'subSpatialParts'
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (part_usage 'propellerMotorAssy4' : 'PropellerMotorAssy' :> 'subSpatialParts'
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value))))
      (comment)
      (part_usage 'camera' : 'Camera' :> 'subSpatialParts'
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'elements' value)))))))
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
        attribute :> differencesOf [1] {
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
            attribute :> intersectionsOf [1] {
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
        part camera : Camera :> subSpatialParts {
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
# EXPECTED
~~~
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'differencesOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cone'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'intersectionsOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'differencesOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cone'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'intersectionsOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'elements'
~~~
# SMG
~~~
(model
  (namespace
    (package 'SimpleQuadcopter'
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'SpatialItems'[unresolved])
      (namespace_import private -> 'ShapeItems'[unresolved])
      (membership_import private -> 'RealFunctions::sqrt'[unresolved])
      (membership_import private -> 'TrigFunctions::pi'[unresolved])
      (membership_import private -> 'TrigFunctions::tan'[unresolved])
      (membership_import private -> 'MeasurementReferences::CoordinateFrame'[unresolved])
      (membership_import private -> 'MeasurementReferences::TranslationRotationSequence'[unresolved])
      (membership_import private -> 'MeasurementReferences::Translation'[unresolved])
      (membership_import private -> 'MeasurementReferences::Rotation'[unresolved])
      (part_usage 'motorShape' : 'SpatialItem'[unresolved]
        (item_usage composite :>> 'shape'[unresolved] : 'Cylinder'[unresolved]
          (reference_usage reference :>> 'radius'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'height'[unresolved]
            (feature_value (=)))))
      (part_def 'Strut' :> 'SpatialItem'[unresolved]
        (part_usage composite 'rawStrut' :> 'subSpatialParts'[unresolved]
          (item_usage composite :>> 'shape'[unresolved] : 'Box'[unresolved]
            (reference_usage reference :>> 'length'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'width'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'height'[unresolved]
              (feature_value (=))))
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'motorCutout' :> 'subSpatialParts'[unresolved]
          (item_usage composite :>> 'shape'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (attribute_usage composite :> 'differencesOf'[unresolved]
          (multiplicity_range [1])
          (item_usage composite :>> 'elements'[unresolved]
            (feature_value (=)))))
      (part_def 'PropellerMotorAssy' :> 'SpatialItem'[unresolved]
        (part_usage composite 'propeller' :> 'subSpatialParts'[unresolved]
          (item_usage composite :>> 'shape'[unresolved] : 'Cylinder'[unresolved]
            (documentation)
            (reference_usage reference :>> 'radius'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'height'[unresolved]
              (feature_value (=))))
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'motor' :> 'subSpatialParts'[unresolved]
          (item_usage composite :>> 'shape'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=)))))))
      (part_def 'Camera' :> 'SpatialItem'[unresolved]
        (part_usage composite 'cameraHousing' :> 'subSpatialParts'[unresolved]
          (item_usage composite :>> 'shape'[unresolved] : 'Cylinder'[unresolved]
            (reference_usage reference :>> 'radius'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'height'[unresolved]
              (feature_value (=)))))
        (item_usage composite 'fieldOfView' :> 'subSpatialParts'[unresolved]
          (documentation)
          (item_usage composite :>> 'shape'[unresolved] : 'Cone'[unresolved]
            (reference_usage reference :>> 'radius'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'height'[unresolved]
              (feature_value (=))))
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=)))))))
      (part_usage 'quadCopter' : 'SpatialItem'[unresolved]
        (attribute_usage composite 'datum' :>> 'coordinateFrame'[unresolved]
          (documentation)
          (reference_usage reference :>> 'mRefs'[unresolved]
            (feature_value (=))))
        (part_usage composite 'mainBody' :> 'subSpatialParts'[unresolved]
          (part_usage composite 'rawBody' :> 'subSpatialParts'[unresolved]
            (item_usage composite :>> 'shape'[unresolved] : 'Box'[unresolved]
              (reference_usage reference :>> 'length'[unresolved]
                (feature_value (=)))
              (reference_usage reference :>> 'width'[unresolved]
                (feature_value (=)))
              (reference_usage reference :>> 'height'[unresolved]
                (feature_value (=))))
            (attribute_usage composite :>> 'coordinateFrame'[unresolved]
              (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
                (reference_usage reference :>> 'elements'[unresolved]
                  (feature_value (=))))))
          (part_usage composite 'cuttingCornersBox' :> 'subSpatialParts'[unresolved]
            (item_usage composite :>> 'shape'[unresolved] : 'Box'[unresolved]
              (reference_usage reference :>> 'length'[unresolved]
                (feature_value (=)))
              (reference_usage reference :>> 'width'[unresolved]
                (feature_value (=)))
              (reference_usage reference :>> 'height'[unresolved]
                (feature_value (=))))
            (attribute_usage composite :>> 'coordinateFrame'[unresolved]
              (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
                (reference_usage reference :>> 'elements'[unresolved]
                  (feature_value (=))))))
          (attribute_usage composite :> 'intersectionsOf'[unresolved]
            (multiplicity_range [1])
            (item_usage composite :>> 'elements'[unresolved]
              (feature_value (=)))))
        (attribute_usage composite 'xStrut' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'yStrut' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'zStrut' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'zPMAssy' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (part_usage composite 'strut1' : 'SimpleQuadcopter::Strut'[part_def] :> 'subSpatialParts'[unresolved]
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'strut2' : 'SimpleQuadcopter::Strut'[part_def] :> 'subSpatialParts'[unresolved]
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'strut3' : 'SimpleQuadcopter::Strut'[part_def] :> 'subSpatialParts'[unresolved]
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'strut4' : 'SimpleQuadcopter::Strut'[part_def] :> 'subSpatialParts'[unresolved]
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'propellerMotorAssy1' : 'SimpleQuadcopter::PropellerMotorAssy'[part_def] :> 'subSpatialParts'[unresolved]
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'propellerMotorAssy2' : 'SimpleQuadcopter::PropellerMotorAssy'[part_def] :> 'subSpatialParts'[unresolved]
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'propellerMotorAssy3' : 'SimpleQuadcopter::PropellerMotorAssy'[part_def] :> 'subSpatialParts'[unresolved]
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'propellerMotorAssy4' : 'SimpleQuadcopter::PropellerMotorAssy'[part_def] :> 'subSpatialParts'[unresolved]
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'camera' : 'SimpleQuadcopter::Camera'[part_def] :> 'subSpatialParts'[unresolved]
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))))))
~~~
