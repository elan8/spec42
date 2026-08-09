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
# FORMAT
~~~sysml
standard library package SpatialFrames {
    doc /*
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
        doc /*
         * DefaultFrameLife is the classifier of the singleton Life of the defaultFrame.
         */
    }

    feature defaultFrame : DefaultFrameLife [1] {
        doc /*
         * defaultFrame is a fixed SpatialFrame used as a universal default.
         */
    }

    abstract struct SpatialFrame specializes Body {
        doc /*
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
        doc /*
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
(model
  (namespace
    (library_package 'SpatialFrames'
      (documentation)
      (namespace_import private -> 'Clocks'[unresolved])
      (membership_import private -> 'ScalarValues::NumericalValue'[unresolved])
      (membership_import private -> 'VectorValues::ThreeVectorValue'[unresolved])
      (membership_import private -> 'VectorValues::CartesianThreeVectorValue'[unresolved])
      (membership_import private -> 'VectorFunctions::isZeroVector'[unresolved])
      (membership_import private -> 'Occurrences::Life'[unresolved])
      (membership_import private -> 'Objects::Body'[unresolved])
      (membership_import private -> 'Objects::Point'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (membership_import private -> 'SequenceFunctions::includes'[unresolved])
      (structure_def 'DefaultFrameLife' :> 'SpatialFrames::SpatialFrame'[structure_def] :> 'Life'[unresolved]
        (multiplicity_range [1])
        (documentation))
      (feature_def 'defaultFrame' : 'SpatialFrames::DefaultFrameLife'[structure_def]
        (multiplicity_range [1])
        (documentation))
      (structure_def abstract 'SpatialFrame' :> 'Body'[unresolved]
        (documentation))
      (function_def abstract 'PositionOf'
        (documentation)
        (feature_def in 'point' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'time' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'frame' : 'SpatialFrames::SpatialFrame'[structure_def]
          (multiplicity_range [1])
          (feature_value (default =)))
        (feature_def in 'clock' : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'positionVector' : 'ThreeVectorValue'[unresolved]
            (multiplicity_range [1])))
        (invariant_def 'positionTimePrecondition'
          (documentation)
          (result_expr_membership))
        (invariant_def 'spacePositionConstraint'
          (documentation)
          (result_expr_membership)))
      (function_def abstract 'CurrentPositionOf'
        (documentation)
        (feature_def in 'point' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'frame' : 'SpatialFrames::SpatialFrame'[structure_def]
          (multiplicity_range [1])
          (feature_value (default =)))
        (feature_def in 'clock' : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'positionVector' : 'ThreeVectorValue'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'DisplacementOf'
        (documentation)
        (feature_def in 'point1' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'point2' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'time' : 'NumericalValue'[unresolved])
        (feature_def in 'frame' : 'SpatialFrames::SpatialFrame'[structure_def]
          (multiplicity_range [1])
          (feature_value (default =)))
        (feature_def in 'clock' : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'displacementVector' : 'ThreeVectorValue'[unresolved]
            (multiplicity_range [1])
            (feature_value (=))))
        (invariant_def 'zeroDisplacementConstraint'
          (documentation)
          (result_expr_membership)))
      (function_def 'CurrentDisplacementOf'
        (documentation)
        (feature_def in 'point1' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'point2' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'frame' : 'SpatialFrames::SpatialFrame'[structure_def]
          (multiplicity_range [1])
          (feature_value (default =)))
        (feature_def in 'clock' : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'displacementVector' : 'ThreeVectorValue'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (structure_def abstract 'CartesianSpatialFrame' :> 'SpatialFrames::SpatialFrame'[structure_def]
        (documentation))
      (function_def abstract 'CartesianPositionOf' :> 'SpatialFrames::PositionOf'[function_def]
        (documentation)
        (feature_def in 'point' : 'Point'[unresolved] :>> 'SpatialFrames::PositionOf::point'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'time' : 'NumericalValue'[unresolved] :>> 'SpatialFrames::PositionOf::time'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'frame' : 'SpatialFrames::CartesianSpatialFrame'[structure_def] :>> 'SpatialFrames::PositionOf::frame'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'clock' : 'Clock'[unresolved] :>> 'SpatialFrames::PositionOf::clock'[feature_def][implied]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'positionVector' : 'CartesianThreeVectorValue'[unresolved] :>> 'positionVector'[feature_def][implied]
            (multiplicity_range [1]))))
      (function_def abstract 'CartesianCurrentPositionOf' :> 'SpatialFrames::CurrentPositionOf'[function_def]
        (documentation)
        (feature_def in 'point' : 'Point'[unresolved] :>> 'SpatialFrames::CurrentPositionOf::point'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'frame' : 'SpatialFrames::CartesianSpatialFrame'[structure_def] :>> 'SpatialFrames::CurrentPositionOf::frame'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'clock' : 'Clock'[unresolved] :>> 'SpatialFrames::CurrentPositionOf::clock'[feature_def][implied]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'positionVector' : 'CartesianThreeVectorValue'[unresolved] :>> 'positionVector'[feature_def][implied]
            (multiplicity_range [1]))))
      (function_def 'CartesianDisplacementOf' :> 'SpatialFrames::DisplacementOf'[function_def]
        (documentation)
        (feature_def in 'point1' : 'Point'[unresolved] :>> 'SpatialFrames::DisplacementOf::point1'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'point2' : 'Point'[unresolved] :>> 'SpatialFrames::DisplacementOf::point2'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'time' : 'NumericalValue'[unresolved] :>> 'SpatialFrames::DisplacementOf::time'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'frame' : 'SpatialFrames::CartesianSpatialFrame'[structure_def] :>> 'SpatialFrames::DisplacementOf::frame'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'clock' : 'Clock'[unresolved] :>> 'SpatialFrames::DisplacementOf::clock'[feature_def][implied]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'displacementVector' : 'CartesianThreeVectorValue'[unresolved] :>> 'displacementVector'[feature_def][implied]
            (multiplicity_range [1]))))
      (function_def 'CartesianCurrentDisplacementOf' :> 'SpatialFrames::CurrentDisplacementOf'[function_def]
        (documentation)
        (feature_def in 'point1' : 'Point'[unresolved] :>> 'SpatialFrames::CurrentDisplacementOf::point1'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'point2' : 'Point'[unresolved] :>> 'SpatialFrames::CurrentDisplacementOf::point2'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'frame' : 'SpatialFrames::CartesianSpatialFrame'[structure_def] :>> 'SpatialFrames::CurrentDisplacementOf::frame'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'clock' : 'Clock'[unresolved] :>> 'SpatialFrames::CurrentDisplacementOf::clock'[feature_def][implied]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'displacementVector' : 'CartesianThreeVectorValue'[unresolved] :>> 'displacementVector'[feature_def][implied]
            (multiplicity_range [1])))))))
~~~
