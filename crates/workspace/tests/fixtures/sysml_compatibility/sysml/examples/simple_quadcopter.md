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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "SimpleQuadcopter"))) (name "SimpleQuadcopter") (declared-name "SimpleQuadcopter")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))) (name "Camera") (declared-name "Camera") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::cameraHousing"))) (name "cameraHousing") (declared-name "cameraHousing") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Camera")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView"))) (name "fieldOfView") (declared-name "fieldOfView") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Camera")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Camera")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::CoordinateFrame"))) (name "CoordinateFrame") (declared-name "CoordinateFrame"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))) (name "PropellerMotorAssy") (declared-name "PropellerMotorAssy") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor"))) (name "motor") (declared-name "motor") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::motor::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller"))) (name "propeller") (declared-name "propeller") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy::propeller::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Rotation"))) (name "Rotation") (declared-name "Rotation"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))) (name "Strut") (declared-name "Strut") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout"))) (name "motorCutout") (declared-name "motorCutout") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::motorCutout::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut"))) (name "rawStrut") (declared-name "rawStrut") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Strut::rawStrut::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::Translation"))) (name "Translation") (declared-name "Translation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::TranslationRotationSequence"))) (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence"))
        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::motorShape"))) (name "motorShape") (declared-name "motorShape") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::pi"))) (name "pi") (declared-name "pi"))
        (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter"))) (name "quadCopter") (declared-name "quadCopter") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (name "camera") (declared-name "camera") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Camera")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::datum"))) (name "datum") (declared-name "datum") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody"))) (name "mainBody") (declared-name "mainBody") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox"))) (name "cuttingCornersBox") (declared-name "cuttingCornersBox") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::cuttingCornersBox::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody"))) (name "rawBody") (declared-name "rawBody") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::mainBody::rawBody::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (name "propellerMotorAssy1") (declared-name "propellerMotorAssy1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (name "propellerMotorAssy2") (declared-name "propellerMotorAssy2") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (name "propellerMotorAssy3") (declared-name "propellerMotorAssy3") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (name "propellerMotorAssy4") (declared-name "propellerMotorAssy4") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (name "strut1") (declared-name "strut1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (name "strut2") (declared-name "strut2") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (name "strut3") (declared-name "strut3") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (name "strut4") (declared-name "strut4") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SimpleQuadcopter::Strut")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (name "xStrut") (declared-name "xStrut") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "49.60")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::xStrut"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (name "yStrut") (declared-name "yStrut") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "24.65")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::yStrut"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (name "zPMAssy") (declared-name "zPMAssy") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 12)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zPMAssy"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (name "zStrut") (declared-name "zStrut") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 25)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::zStrut"))) (role feature-value))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::sqrt"))) (name "sqrt") (declared-name "sqrt"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SimpleQuadcopter::tan"))) (name "tan") (declared-name "tan"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView::_documentation"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::Camera::fieldOfView"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::camera"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::Camera"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy1"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy2"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy3"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::propellerMotorAssy4"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::PropellerMotorAssy"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut1"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut2"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut3"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SimpleQuadcopter::quadCopter::strut4"))) (to (node (document "d0") (qualified-name "SimpleQuadcopter::Strut"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/simple_quadcopter.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 4) (end 1 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 4) (end 2 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 4) (end 3 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 4) (end 4 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 4) (end 5 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 4) (end 6 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 4) (end 7 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 4) (end 8 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 4) (end 9 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 4) (end 10 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 4) (end 11 51))
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
        (range (start 20 4) (end 20 1337))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 31 12) (end 31 228))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 41 12) (end 41 219))
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
        (range (start 54 4) (end 54 1158))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 64 12) (end 64 219))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 73 12) (end 73 218))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 83 4) (end 83 1233))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 103 12) (end 103 224))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 22) (end 113 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 114 8) (end 114 519))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 132 16) (end 132 248))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 146 16) (end 146 342))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 155 12) (end 155 329))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
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
        (code "unknown_unit_symbol")
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
        (code "unknown_unit_symbol")
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
        (code "unknown_unit_symbol")
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 170 12) (end 170 323))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 178 12) (end 178 325))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 186 12) (end 186 326))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 194 12) (end 194 325))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 203 12) (end 203 324))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 211 12) (end 211 326))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 219 12) (end 219 327))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 227 12) (end 227 326))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 237 12) (end 237 297))
      )
    )
  )
)
~~~
