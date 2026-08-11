# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/SpatialFrames
type=file
~~~
# SOURCE
~~~kerml
standard library package SpatialFrames {
    doc
    /*
     * This package models spatial frames of reference for quantifying the position of points 
     * in a three-dimensional space. 
     */

    private import Clocks::*;
    private import ScalarValues::NumericalValue;
    private import VectorValues::ThreeVectorValue;
    private import VectorValues::CartesianThreeVectorValue;
    private import VectorFunctions::isZeroVector;
    private import Occurrences::Life;
    private import Objects::Body;
    private import Objects::Point;
    private import ControlFunctions::forAll;
    private import SequenceFunctions::includes;
    
    private struct DefaultFrameLife[1] :> SpatialFrame, Life {
        doc
        /*
         * DefaultFrameLife is the classifier of the singleton Life of the defaultFrame.
         */
    }
    
    feature defaultFrame : DefaultFrameLife[1] {
        doc
        /*
         * defaultFrame is a fixed SpatialFrame used as a universal default.
         */
    }
    
    abstract struct SpatialFrame specializes Body {
        doc
        /*
         * A SpatialFrame is a three-dimensional Body that provides a spatial extent that 
         * acts as a frame of reference for defining the physical position and displacement 
         * vectors of Points over time.
         */
    }
    
    abstract function PositionOf {
        doc
        /*
         * The PositionOf a Point relative to a SpatialFrame, at a specific time relative to a given
         * Clock, as a positionVector that is a ThreeVectorValue.
         */

        in point : Point[1];
        in time : NumericalValue[1];
        in 'frame' : SpatialFrame[1] default defaultFrame;
        in clock : Clock[1] default 'frame'.localClock;
        return positionVector : ThreeVectorValue[1];

        inv positionTimePrecondition {
            doc
            /*
             * The given point must exist at the given time.
             */

            TimeOf(point.startShot) <= time and
            time <= TimeOf(point.endShot)
        }

        inv spacePositionConstraint {
            doc
            /*
             * The result positionVector is equal to the PositionOf the Point spaceShot of the
             * frame that encloses the given point, at the given time.
             */

            ('frame'.spaceShots as Point)->forAll{in p : Point;
                p.spaceTimeEnclosedOccurrences->includes(point) implies
                    positionVector == PositionOf(p, time, 'frame')
            }
        }
    }

    abstract function CurrentPositionOf {
        doc
        /*
         * The CurrentPositionOf a Point relative to a SpatialFrame and a Clock is the PositionOf
         * the Point relative to the SpatialFrame at the currentTime of the Clock.
         */

        in point : Point[1];
        in 'frame' : SpatialFrame[1] default defaultFrame;
        in clock : Clock[1] default 'frame'.localClock;
        return positionVector : ThreeVectorValue[1] =
            PositionOf(point, clock.currentTime, 'frame', clock);
    }

    function DisplacementOf {
        doc
        /*
         * The DisplacementOf two Points relative to a SpatialFrame, at a specific time relative to a
         * given Clock, is the displacementVector computed as the difference between the PositionOf the
         * first Point and PositionOf the second Point, relative to that SpatialFrame, at that time.
         */

        in point1 : Point[1];
        in point2 : Point[1];
        in time : NumericalValue;
        in 'frame' : SpatialFrame[1] default defaultFrame;
        in clock : Clock[1] default 'frame'.localClock;
        return displacementVector : ThreeVectorValue[1] =
            PositionOf(point2, time, 'frame', clock) - PositionOf(point1, time, 'frame', clock);

        inv zeroDisplacementConstraint {
        doc
        /*
         * If either point1 or point2 occurs within the other, then the displacementVector is
         * the zero vector.
         */

            (point1.spaceTimeEnclosedOccurrences->includes(point2) or
            point2.spaceTimeEnclosedOccurrences->includes(point1)) implies
                isZeroVector(displacementVector)
        }
    }

    function CurrentDisplacementOf {
        doc
        /*
         * The CurrentDisplacementOf two Points relative to a SpatialFrame and Clock is the DisplacementOf
         * the Points relative to the SpatialFrame at the currentTime of the Clock.
         */

        in point1 : Point[1];
        in point2 : Point[1];
        in 'frame' : SpatialFrame[1] default defaultFrame;
        in clock : Clock[1] default 'frame'.localClock;
        return displacementVector : ThreeVectorValue[1] =
            DisplacementOf(point1, point2, clock.currentTime, 'frame', clock);
    }
    
    abstract struct CartesianSpatialFrame :> SpatialFrame {
        doc
        /*
         * A CartesianSpatialFrame is a SpatialFrame relative to which all position and displacement
         * vectors can be represented as CartesianThreeVectorValues.
         */
    }
    
    abstract function CartesianPositionOf :> PositionOf {
        doc
        /*
         * The PositionOf a Point relative to a CartesianSpatialFrame is a CartesianThreeVectorValue.
         */

        in point : Point[1];
        in time : NumericalValue[1];
        in 'frame' : CartesianSpatialFrame[1];
        in clock : Clock[1] default 'frame'.localClock;
        return positionVector : CartesianThreeVectorValue[1];
    }

    abstract function CartesianCurrentPositionOf :> CurrentPositionOf {
        doc
        /*
         * The CurrentPositionOf a Point relative to a CartesianSpatialFrame is a CartesianThreeVectorValue.
         */

        in point : Point[1];
        in 'frame' : CartesianSpatialFrame[1];
        in clock : Clock[1] default 'frame'.localClock;
        return positionVector : CartesianThreeVectorValue[1];
    }

    function CartesianDisplacementOf :> DisplacementOf {
        doc
        /*
         * The DisplacementOf two Points relative to a CartesianSpatialFrame is a CartesianThreeVectorValue.
         */

        in point1 : Point[1];
        in point2 : Point[1];
        in time : NumericalValue[1];
        in 'frame' : CartesianSpatialFrame[1];
        in clock : Clock[1] default 'frame'.localClock;
        return displacementVector : CartesianThreeVectorValue[1];
    }

    function CartesianCurrentDisplacementOf :> CurrentDisplacementOf {
        doc
        /*
         * The CurrentDisplacementOf two Points relative to a CartesianSpatialFrame is a CartesianThreeVectorValue.
         */

        in point1 : Point[1];
        in point2 : Point[1];
        in 'frame' : CartesianSpatialFrame[1];
        in clock : Clock[1] default 'frame'.localClock;
        return displacementVector : CartesianThreeVectorValue[1];
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "spatial_frames.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 19) (end 11 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 19) (end 12 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 19) (end 13 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 19) (end 14 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 19) (end 16 46))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwStruct,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,UnrestrictedName,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,OpenParen,Ident,Dot,Ident,CloseParen,LtEq,Ident,KwAnd,
Ident,LtEq,Ident,OpenParen,Ident,Dot,Ident,CloseParen,
CloseCurly,
KwInv,Ident,OpenCurly,
KwDoc,
RegularComment,
OpenParen,UnrestrictedName,Dot,Ident,KwAs,Ident,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,Dot,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,KwImplies,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,UnrestrictedName,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,UnrestrictedName,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
Ident,OpenParen,Ident,Comma,Ident,Dot,Ident,Comma,UnrestrictedName,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,UnrestrictedName,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
Ident,OpenParen,Ident,Comma,Ident,Comma,UnrestrictedName,Comma,Ident,CloseParen,Minus,Ident,OpenParen,Ident,Comma,Ident,Comma,UnrestrictedName,Comma,Ident,CloseParen,Semicolon,
KwInv,Ident,OpenCurly,
KwDoc,
RegularComment,
OpenParen,Ident,Dot,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,KwOr,
Ident,Dot,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,CloseParen,KwImplies,
Ident,OpenParen,Ident,CloseParen,
CloseCurly,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,UnrestrictedName,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Dot,Ident,Comma,UnrestrictedName,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwAbstract,KwStruct,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFunction,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,UnrestrictedName,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,UnrestrictedName,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,UnrestrictedName,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,UnrestrictedName,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'SpatialFrames'
    (documentation)
    (import_decl private 'Clocks::*')
    (import_decl private 'ScalarValues::NumericalValue')
    (import_decl private 'VectorValues::ThreeVectorValue')
    (import_decl private 'VectorValues::CartesianThreeVectorValue')
    (import_decl private 'VectorFunctions::isZeroVector')
    (import_decl private 'Occurrences::Life')
    (import_decl private 'Objects::Body')
    (import_decl private 'Objects::Point')
    (import_decl private 'ControlFunctions::forAll')
    (import_decl private 'SequenceFunctions::includes')
    (structure_def private 'DefaultFrameLife' multiplicity     (multiplicity_range) :> 'SpatialFrame', 'Life'
      (documentation))
    (feature_def 'defaultFrame' : 'DefaultFrameLife' multiplicity
      (documentation))
    (structure_def abstract 'SpatialFrame' :> 'Body'
      (documentation))
    (function_def
      (documentation)
      (feature_def in 'point' : 'Point' multiplicity)
      (feature_def in 'time' : 'NumericalValue' multiplicity)
      (feature_def in ''frame'' : 'SpatialFrame' multiplicity value)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member)
      (invariant_def
        (documentation)
        (result_expr_member))
      (invariant_def
        (documentation)
        (result_expr_member)))
    (function_def
      (documentation)
      (feature_def in 'point' : 'Point' multiplicity)
      (feature_def in ''frame'' : 'SpatialFrame' multiplicity value)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'point1' : 'Point' multiplicity)
      (feature_def in 'point2' : 'Point' multiplicity)
      (feature_def in 'time' : 'NumericalValue')
      (feature_def in ''frame'' : 'SpatialFrame' multiplicity value)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member)
      (invariant_def
        (documentation)
        (result_expr_member)))
    (function_def
      (documentation)
      (feature_def in 'point1' : 'Point' multiplicity)
      (feature_def in 'point2' : 'Point' multiplicity)
      (feature_def in ''frame'' : 'SpatialFrame' multiplicity value)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (structure_def abstract 'CartesianSpatialFrame' :> 'SpatialFrame'
      (documentation))
    (function_def
      (documentation)
      (feature_def in 'point' : 'Point' multiplicity)
      (feature_def in 'time' : 'NumericalValue' multiplicity)
      (feature_def in ''frame'' : 'CartesianSpatialFrame' multiplicity)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'point' : 'Point' multiplicity)
      (feature_def in ''frame'' : 'CartesianSpatialFrame' multiplicity)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'point1' : 'Point' multiplicity)
      (feature_def in 'point2' : 'Point' multiplicity)
      (feature_def in 'time' : 'NumericalValue' multiplicity)
      (feature_def in ''frame'' : 'CartesianSpatialFrame' multiplicity)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'point1' : 'Point' multiplicity)
      (feature_def in 'point2' : 'Point' multiplicity)
      (feature_def in ''frame'' : 'CartesianSpatialFrame' multiplicity)
      (feature_def in 'clock' : 'Clock' multiplicity value)
      (return_member))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Life'
semantic.unresolved_name 'Body'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'CartesianThreeVectorValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Life'
semantic.unresolved_name 'Body'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'CartesianThreeVectorValue'
~~~
# FORMAT
~~~sysml
standard library package SpatialFrames {
    doc
    /*
     * This package models spatial frames of reference for quantifying the position of points 
     * in a three-dimensional space. 
     */

    private import Clocks::*;
    private import ScalarValues::NumericalValue;
    private import VectorValues::ThreeVectorValue;
    private import VectorValues::CartesianThreeVectorValue;
    private import VectorFunctions::isZeroVector;
    private import Occurrences::Life;
    private import Objects::Body;
    private import Objects::Point;
    private import ControlFunctions::forAll;
    private import SequenceFunctions::includes;
    
    private struct DefaultFrameLife[1] :> SpatialFrame, Life {
        doc
        /*
         * DefaultFrameLife is the classifier of the singleton Life of the defaultFrame.
         */
    }
    
    feature defaultFrame : DefaultFrameLife[1] {
        doc
        /*
         * defaultFrame is a fixed SpatialFrame used as a universal default.
         */
    }
    
    abstract struct SpatialFrame specializes Body {
        doc
        /*
         * A SpatialFrame is a three-dimensional Body that provides a spatial extent that 
         * acts as a frame of reference for defining the physical position and displacement 
         * vectors of Points over time.
         */
    }
    
    abstract function PositionOf {
        doc
        /*
         * The PositionOf a Point relative to a SpatialFrame, at a specific time relative to a given
         * Clock, as a positionVector that is a ThreeVectorValue.
         */

        in point : Point[1];
        in time : NumericalValue[1];
        in 'frame' : SpatialFrame[1] default defaultFrame;
        in clock : Clock[1] default 'frame'.localClock;
        return positionVector : ThreeVectorValue[1];

        inv positionTimePrecondition {
            doc
            /*
             * The given point must exist at the given time.
             */

            TimeOf(point.startShot) <= time and
            time <= TimeOf(point.endShot)
        }

        inv spacePositionConstraint {
            doc
            /*
             * The result positionVector is equal to the PositionOf the Point spaceShot of the
             * frame that encloses the given point, at the given time.
             */

            ('frame'.spaceShots as Point)->forAll{in p : Point;
                p.spaceTimeEnclosedOccurrences->includes(point) implies
                    positionVector == PositionOf(p, time, 'frame')
            }
        }
    }

    abstract function CurrentPositionOf {
        doc
        /*
         * The CurrentPositionOf a Point relative to a SpatialFrame and a Clock is the PositionOf
         * the Point relative to the SpatialFrame at the currentTime of the Clock.
         */

        in point : Point[1];
        in 'frame' : SpatialFrame[1] default defaultFrame;
        in clock : Clock[1] default 'frame'.localClock;
        return positionVector : ThreeVectorValue[1] =
            PositionOf(point, clock.currentTime, 'frame', clock);
    }

    function DisplacementOf {
        doc
        /*
         * The DisplacementOf two Points relative to a SpatialFrame, at a specific time relative to a
         * given Clock, is the displacementVector computed as the difference between the PositionOf the
         * first Point and PositionOf the second Point, relative to that SpatialFrame, at that time.
         */

        in point1 : Point[1];
        in point2 : Point[1];
        in time : NumericalValue;
        in 'frame' : SpatialFrame[1] default defaultFrame;
        in clock : Clock[1] default 'frame'.localClock;
        return displacementVector : ThreeVectorValue[1] =
            PositionOf(point2, time, 'frame', clock) - PositionOf(point1, time, 'frame', clock);

        inv zeroDisplacementConstraint {
        doc
        /*
         * If either point1 or point2 occurs within the other, then the displacementVector is
         * the zero vector.
         */

            (point1.spaceTimeEnclosedOccurrences->includes(point2) or
            point2.spaceTimeEnclosedOccurrences->includes(point1)) implies
                isZeroVector(displacementVector)
        }
    }

    function CurrentDisplacementOf {
        doc
        /*
         * The CurrentDisplacementOf two Points relative to a SpatialFrame and Clock is the DisplacementOf
         * the Points relative to the SpatialFrame at the currentTime of the Clock.
         */

        in point1 : Point[1];
        in point2 : Point[1];
        in 'frame' : SpatialFrame[1] default defaultFrame;
        in clock : Clock[1] default 'frame'.localClock;
        return displacementVector : ThreeVectorValue[1] =
            DisplacementOf(point1, point2, clock.currentTime, 'frame', clock);
    }
    
    abstract struct CartesianSpatialFrame :> SpatialFrame {
        doc
        /*
         * A CartesianSpatialFrame is a SpatialFrame relative to which all position and displacement
         * vectors can be represented as CartesianThreeVectorValues.
         */
    }
    
    abstract function CartesianPositionOf :> PositionOf {
        doc
        /*
         * The PositionOf a Point relative to a CartesianSpatialFrame is a CartesianThreeVectorValue.
         */

        in point : Point[1];
        in time : NumericalValue[1];
        in 'frame' : CartesianSpatialFrame[1];
        in clock : Clock[1] default 'frame'.localClock;
        return positionVector : CartesianThreeVectorValue[1];
    }

    abstract function CartesianCurrentPositionOf :> CurrentPositionOf {
        doc
        /*
         * The CurrentPositionOf a Point relative to a CartesianSpatialFrame is a CartesianThreeVectorValue.
         */

        in point : Point[1];
        in 'frame' : CartesianSpatialFrame[1];
        in clock : Clock[1] default 'frame'.localClock;
        return positionVector : CartesianThreeVectorValue[1];
    }

    function CartesianDisplacementOf :> DisplacementOf {
        doc
        /*
         * The DisplacementOf two Points relative to a CartesianSpatialFrame is a CartesianThreeVectorValue.
         */

        in point1 : Point[1];
        in point2 : Point[1];
        in time : NumericalValue[1];
        in 'frame' : CartesianSpatialFrame[1];
        in clock : Clock[1] default 'frame'.localClock;
        return displacementVector : CartesianThreeVectorValue[1];
    }

    function CartesianCurrentDisplacementOf :> CurrentDisplacementOf {
        doc
        /*
         * The CurrentDisplacementOf two Points relative to a CartesianSpatialFrame is a CartesianThreeVectorValue.
         */

        in point1 : Point[1];
        in point2 : Point[1];
        in 'frame' : CartesianSpatialFrame[1];
        in clock : Clock[1] default 'frame'.localClock;
        return displacementVector : CartesianThreeVectorValue[1];
    }

}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4132489f2c2297db203fcc5585e68ed65bc1ba3b1786e495c4cbe8638b1b87e8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SpatialFrames"))) (kind "package") (name "SpatialFrames") (declared-name "SpatialFrames") (range (start (line 0) (character 0)) (end (line 0) (character 6871))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 4)) (end (line 7) (character 29))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Clocks::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 25))))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::Body"))) (kind "import") (name "Body") (declared-name "Body") (range (start (line 13) (character 4)) (end (line 13) (character 33))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Body") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 19)) (end (line 13) (character 32))))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf"))) (kind "kermlDecl") (name "CartesianCurrentDisplacementOf") (declared-name "CartesianCurrentDisplacementOf") (range (start (line 183) (character 4)) (end (line 183) (character 457))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianCurrentPositionOf"))) (kind "kermlDecl") (name "CartesianCurrentPositionOf") (declared-name "CartesianCurrentPositionOf") (range (start (line 157) (character 4)) (end (line 157) (character 416))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianDisplacementOf"))) (kind "kermlDecl") (name "CartesianDisplacementOf") (declared-name "CartesianDisplacementOf") (range (start (line 169) (character 4)) (end (line 169) (character 473))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianPositionOf"))) (kind "kermlDecl") (name "CartesianPositionOf") (declared-name "CartesianPositionOf") (range (start (line 144) (character 4)) (end (line 144) (character 432))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (kind "classifier decl") (name "CartesianSpatialFrame") (declared-name "CartesianSpatialFrame") (range (start (line 136) (character 4)) (end (line 136) (character 270))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianThreeVectorValue"))) (kind "import") (name "CartesianThreeVectorValue") (declared-name "CartesianThreeVectorValue") (range (start (line 10) (character 4)) (end (line 10) (character 59))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorValues::CartesianThreeVectorValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 19)) (end (line 10) (character 58))))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CurrentDisplacementOf"))) (kind "kermlDecl") (name "CurrentDisplacementOf") (declared-name "CurrentDisplacementOf") (range (start (line 121) (character 4)) (end (line 121) (character 581))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CurrentPositionOf"))) (kind "kermlDecl") (name "CurrentPositionOf") (declared-name "CurrentPositionOf") (range (start (line 78) (character 4)) (end (line 78) (character 528))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::DefaultFrameLife1"))) (kind "classifier decl") (name "DefaultFrameLife1") (declared-name "DefaultFrameLife1") (range (start (line 18) (character 4)) (end (line 18) (character 192))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::DisplacementOf"))) (kind "kermlDecl") (name "DisplacementOf") (declared-name "DisplacementOf") (range (start (line 92) (character 4)) (end (line 92) (character 1146))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::Life"))) (kind "import") (name "Life") (declared-name "Life") (range (start (line 12) (character 4)) (end (line 12) (character 37))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Life") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 19)) (end (line 12) (character 36))))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::NumericalValue"))) (kind "import") (name "NumericalValue") (declared-name "NumericalValue") (range (start (line 8) (character 4)) (end (line 8) (character 48))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::NumericalValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 47))))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::Point"))) (kind "import") (name "Point") (declared-name "Point") (range (start (line 14) (character 4)) (end (line 14) (character 34))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Point") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 33))))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::PositionOf"))) (kind "kermlDecl") (name "PositionOf") (declared-name "PositionOf") (range (start (line 41) (character 4)) (end (line 41) (character 1206))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::SpatialFrame"))) (kind "classifier decl") (name "SpatialFrame") (declared-name "SpatialFrame") (range (start (line 32) (character 4)) (end (line 32) (character 316))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::ThreeVectorValue"))) (kind "import") (name "ThreeVectorValue") (declared-name "ThreeVectorValue") (range (start (line 9) (character 4)) (end (line 9) (character 50))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorValues::ThreeVectorValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 49))))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 6871))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::defaultFrame"))) (kind "feature decl") (name "defaultFrame") (declared-name "defaultFrame") (range (start (line 25) (character 4)) (end (line 25) (character 166))) (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 15) (character 4)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 43))))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::includes"))) (kind "import") (name "includes") (declared-name "includes") (range (start (line 16) (character 4)) (end (line 16) (character 47))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 46))))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::isZeroVector"))) (kind "import") (name "isZeroVector") (declared-name "isZeroVector") (range (start (line 11) (character 4)) (end (line 11) (character 49))) (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorFunctions::isZeroVector") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 19)) (end (line 11) (character 48))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Clocks::*") (range (start (line 7) (character 19)) (end (line 7) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::Body"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Body") (range (start (line 13) (character 19)) (end (line 13) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::CartesianThreeVectorValue"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorValues::CartesianThreeVectorValue") (range (start (line 10) (character 19)) (end (line 10) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::Life"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Life") (range (start (line 12) (character 19)) (end (line 12) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::NumericalValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::NumericalValue") (range (start (line 8) (character 19)) (end (line 8) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::Point"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Point") (range (start (line 14) (character 19)) (end (line 14) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::ThreeVectorValue"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorValues::ThreeVectorValue") (range (start (line 9) (character 19)) (end (line 9) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 15) (character 19)) (end (line 15) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (range (start (line 16) (character 19)) (end (line 16) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::isZeroVector"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorFunctions::isZeroVector") (range (start (line 11) (character 19)) (end (line 11) (character 48))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
