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
  (document "memory://snapshot/spatial_frames.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 28))
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
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 56) (end 18 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 32 45) (end 32 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 19) (end 48 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 49 18) (end 49 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 19) (end 51 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 36) (end 51 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 32) (end 52 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 54 8) (end 62 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 64 8) (end 75 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 19) (end 85 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 87 19) (end 87 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 36) (end 87 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 88 32) (end 88 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 89 30) (end 89 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 20) (end 100 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 20) (end 101 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 102 18) (end 102 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 19) (end 104 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 104 36) (end 104 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 36) (end 105 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 108 8) (end 118 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 128 20) (end 128 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 129 20) (end 129 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 19) (end 131 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 131 36) (end 131 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 132 36) (end 132 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 133 43) (end 133 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 150 19) (end 150 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 151 18) (end 151 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 153 19) (end 153 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 153 36) (end 153 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 154 32) (end 154 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 19) (end 163 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 19) (end 165 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 165 36) (end 165 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 32) (end 166 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 175 20) (end 175 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 176 20) (end 176 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 177 18) (end 177 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 179 19) (end 179 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 179 36) (end 179 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 180 36) (end 180 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 20) (end 189 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 190 20) (end 190 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 192 19) (end 192 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 192 36) (end 192 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 193 36) (end 193 61))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:a931150e35ae59d64afb397f490c305bc513e2c7dd756877614718736f5eeda7") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Clocks") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::NumericalValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "VectorValues::ThreeVectorValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "VectorValues::CartesianThreeVectorValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "VectorFunctions::isZeroVector") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Life") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::Body") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::Point") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::includes") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CurrentDisplacementOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "frame::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::displacementVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianThreeVectorValue"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::frame"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianSpatialFrame") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::point1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::point2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CurrentPositionOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "frame::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::frame"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianSpatialFrame") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::point"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::positionVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianThreeVectorValue"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DisplacementOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "frame::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::displacementVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianThreeVectorValue"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::frame"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianSpatialFrame") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::point1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::point2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::time"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PositionOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "frame::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::frame"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianSpatialFrame") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::point"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::positionVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianThreeVectorValue"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::time"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "frame::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThreeVectorValue")) (expressionOperand (reference "point1")) (expressionOperand (reference "point2")) (expressionOperand (reference "frame")) (expressionOperand (reference "clock")) (memberAccessOperand (reference "clock::currentTime")) (invocationCallee (reference "DisplacementOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialFrame") (direction in)) (expressionOperand (reference "defaultFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "frame::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialFrame") (direction in)) (expressionOperand (reference "defaultFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::point"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThreeVectorValue")) (expressionOperand (reference "point")) (expressionOperand (reference "frame")) (expressionOperand (reference "clock")) (memberAccessOperand (reference "clock::currentTime")) (invocationCallee (reference "PositionOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife"))) (kind kerml-classifier) (membership (kind owning) (visibility private)) (authored (membership (kind owning) (visibility private)) (relationships (specialization (reference "SpatialFrame")) (specialization (reference "Life"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "frame::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThreeVectorValue")) (expressionOperand (reference "point2")) (expressionOperand (reference "time")) (expressionOperand (reference "frame")) (expressionOperand (reference "clock")) (expressionOperand (reference "point1")) (expressionOperand (reference "time")) (expressionOperand (reference "frame")) (expressionOperand (reference "clock")) (invocationCallee (reference "PositionOf")) (invocationCallee (reference "PositionOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialFrame") (direction in)) (expressionOperand (reference "defaultFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "frame::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialFrame") (direction in)) (expressionOperand (reference "defaultFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::point"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::positionVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThreeVectorValue"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::time"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Body"))))
    (declaration (id (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DefaultFrameLife"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Clocks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "VectorValues::ThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "VectorValues::CartesianThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "VectorFunctions::isZeroVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::Body")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::includes")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf"))) (kind specialization) (ordinal 0))
      (authored-target "CurrentDisplacementOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "frame::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::point1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::point2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf"))) (kind specialization) (ordinal 0))
      (authored-target "CurrentPositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "frame::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::point"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::positionVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf"))) (kind specialization) (ordinal 0))
      (authored-target "DisplacementOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "frame::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::point1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::point2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::time"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf"))) (kind specialization) (ordinal 0))
      (authored-target "PositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "frame::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianSpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::point"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::positionVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::time"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "frame::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 0))
      (authored-target "point1")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point1")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 1))
      (authored-target "point2")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point2")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 2))
      (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 3))
      (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::clock")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "clock::currentTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind invocationCallee) (ordinal 0))
      (authored-target "DisplacementOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (kind expressionOperand) (ordinal 0))
      (authored-target "defaultFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "frame::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (kind expressionOperand) (ordinal 0))
      (authored-target "defaultFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::point"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind expressionOperand) (ordinal 0))
      (authored-target "point")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::point")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind expressionOperand) (ordinal 1))
      (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind expressionOperand) (ordinal 2))
      (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::clock")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "clock::currentTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind invocationCallee) (ordinal 0))
      (authored-target "PositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife"))) (kind specialization) (ordinal 1))
      (authored-target "Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "frame::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 0))
      (authored-target "point2")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point2")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 1))
      (authored-target "time")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 2))
      (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 3))
      (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 4))
      (authored-target "point1")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point1")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 5))
      (authored-target "time")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 6))
      (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 7))
      (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind invocationCallee) (ordinal 0))
      (authored-target "PositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind invocationCallee) (ordinal 1))
      (authored-target "PositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (kind expressionOperand) (ordinal 0))
      (authored-target "defaultFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "frame::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (kind expressionOperand) (ordinal 0))
      (authored-target "defaultFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::point"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::positionVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::time"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame"))) (kind specialization) (ordinal 0))
      (authored-target "Body")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame"))) (kind featureTyping) (ordinal 0))
      (authored-target "DefaultFrameLife")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 3)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::point"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 3)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 4)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 5)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 6)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 7)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind invocationCallee) (ordinal 1)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::clock"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::clock"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::point1"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point1"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::point2"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point2"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::clock"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::clock"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::point"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::point"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::positionVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::clock"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::displacementVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::point1"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point1"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::point2"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point2"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::time"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::clock"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::clock"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::frame"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::point"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::point"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::positionVector"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::positionVector"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::time"))) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::time"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/spatial_frames.md") (range (start 7 19) (end 7 28)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Clocks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 8 19) (end 8 47)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 9 19) (end 9 49)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "VectorValues::ThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 10 19) (end 10 58)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "VectorValues::CartesianThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 11 19) (end 11 48)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "VectorFunctions::isZeroVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 12 19) (end 12 36)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 13 19) (end 13 32)) (probe (position 13 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::Body")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 14 19) (end 14 33)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 15 19) (end 15 43)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 16 19) (end 16 46)) (probe (position 16 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::includes")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 183 47) (end 183 68)) (probe (position 183 47))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf"))) (kind specialization) (ordinal 0) (authored-target "CurrentDisplacementOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 192 19) (end 192 24)) (probe (position 192 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 192 36) (end 192 54)) (probe (position 192 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "frame::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 193 36) (end 193 61)) (probe (position 193 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 191 21) (end 191 42)) (probe (position 191 21))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::frame"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 189 20) (end 189 25)) (probe (position 189 20))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::point1"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 190 20) (end 190 25)) (probe (position 190 20))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf::point2"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 157 52) (end 157 69)) (probe (position 157 52))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf"))) (kind specialization) (ordinal 0) (authored-target "CurrentPositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 165 19) (end 165 24)) (probe (position 165 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 165 36) (end 165 54)) (probe (position 165 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "frame::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 164 21) (end 164 42)) (probe (position 164 21))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::frame"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 163 19) (end 163 24)) (probe (position 163 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::point"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 166 32) (end 166 57)) (probe (position 166 32))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianCurrentPositionOf::positionVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 169 40) (end 169 54)) (probe (position 169 40))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf"))) (kind specialization) (ordinal 0) (authored-target "DisplacementOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 179 19) (end 179 24)) (probe (position 179 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 179 36) (end 179 54)) (probe (position 179 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "frame::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 180 36) (end 180 61)) (probe (position 180 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 178 21) (end 178 42)) (probe (position 178 21))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::frame"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 175 20) (end 175 25)) (probe (position 175 20))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::point1"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 176 20) (end 176 25)) (probe (position 176 20))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::point2"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 177 18) (end 177 32)) (probe (position 177 18))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianDisplacementOf::time"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 144 45) (end 144 55)) (probe (position 144 45))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf"))) (kind specialization) (ordinal 0) (authored-target "PositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 153 19) (end 153 24)) (probe (position 153 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 153 36) (end 153 54)) (probe (position 153 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "frame::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 152 21) (end 152 42)) (probe (position 152 21))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::frame"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianSpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 150 19) (end 150 24)) (probe (position 150 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::point"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 154 32) (end 154 57)) (probe (position 154 32))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::positionVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 151 18) (end 151 32)) (probe (position 151 18))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianPositionOf::time"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 136 45) (end 136 57)) (probe (position 136 45))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (kind specialization) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 131 19) (end 131 24)) (probe (position 131 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 131 36) (end 131 54)) (probe (position 131 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "frame::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 132 36) (end 132 52)) (probe (position 132 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0) (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 133 27) (end 133 33)) (probe (position 133 27))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 0) (authored-target "point1")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point1")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 133 35) (end 133 41)) (probe (position 133 35))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 1) (authored-target "point2")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point2")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 133 62) (end 133 69)) (probe (position 133 62))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 2) (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 133 71) (end 133 76)) (probe (position 133 71))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 3) (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::clock")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 133 43) (end 133 60)) (probe (position 133 43))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind memberAccessOperand) (ordinal 0) (authored-target "clock::currentTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 133 12) (end 133 26)) (probe (position 133 12))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::displacementVector"))) (kind invocationCallee) (ordinal 0) (authored-target "DisplacementOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 130 21) (end 130 33)) (probe (position 130 21))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 130 45) (end 130 57)) (probe (position 130 45))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::frame"))) (kind expressionOperand) (ordinal 0) (authored-target "defaultFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 128 20) (end 128 25)) (probe (position 128 20))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point1"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 129 20) (end 129 25)) (probe (position 129 20))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentDisplacementOf::point2"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 87 19) (end 87 24)) (probe (position 87 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 87 36) (end 87 54)) (probe (position 87 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "frame::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 86 21) (end 86 33)) (probe (position 86 21))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 86 45) (end 86 57)) (probe (position 86 45))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame"))) (kind expressionOperand) (ordinal 0) (authored-target "defaultFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 85 19) (end 85 24)) (probe (position 85 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::point"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 88 32) (end 88 48)) (probe (position 88 32))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind featureTyping) (ordinal 0) (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 89 23) (end 89 28)) (probe (position 89 23))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind expressionOperand) (ordinal 0) (authored-target "point")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::point")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 89 49) (end 89 56)) (probe (position 89 49))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind expressionOperand) (ordinal 1) (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::frame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 89 58) (end 89 63)) (probe (position 89 58))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind expressionOperand) (ordinal 2) (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::clock")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 89 30) (end 89 47)) (probe (position 89 30))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind memberAccessOperand) (ordinal 0) (authored-target "clock::currentTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 89 12) (end 89 22)) (probe (position 89 12))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::CurrentPositionOf::positionVector"))) (kind invocationCallee) (ordinal 0) (authored-target "PositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 18 42) (end 18 54)) (probe (position 18 42))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife"))) (kind specialization) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 18 56) (end 18 60)) (probe (position 18 56))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife"))) (kind specialization) (ordinal 1) (authored-target "Life")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 104 19) (end 104 24)) (probe (position 104 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 104 36) (end 104 54)) (probe (position 104 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "frame::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 105 36) (end 105 52)) (probe (position 105 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0) (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 23) (end 106 29)) (probe (position 106 23))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 0) (authored-target "point2")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point2")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 31) (end 106 35)) (probe (position 106 31))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 1) (authored-target "time")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 37) (end 106 44)) (probe (position 106 37))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 2) (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 46) (end 106 51)) (probe (position 106 46))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 3) (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 66) (end 106 72)) (probe (position 106 66))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 4) (authored-target "point1")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point1")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 74) (end 106 78)) (probe (position 106 74))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 5) (authored-target "time")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 80) (end 106 87)) (probe (position 106 80))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 6) (authored-target "frame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 89) (end 106 94)) (probe (position 106 89))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind expressionOperand) (ordinal 7) (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::clock")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 12) (end 106 22)) (probe (position 106 12))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind invocationCallee) (ordinal 0) (authored-target "PositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 106 55) (end 106 65)) (probe (position 106 55))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::displacementVector"))) (kind invocationCallee) (ordinal 1) (authored-target "PositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 103 21) (end 103 33)) (probe (position 103 21))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 103 45) (end 103 57)) (probe (position 103 45))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::frame"))) (kind expressionOperand) (ordinal 0) (authored-target "defaultFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 100 20) (end 100 25)) (probe (position 100 20))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point1"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 101 20) (end 101 25)) (probe (position 101 20))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::point2"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 102 18) (end 102 32)) (probe (position 102 18))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DisplacementOf::time"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 51 19) (end 51 24)) (probe (position 51 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 51 36) (end 51 54)) (probe (position 51 36))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "frame::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 50 21) (end 50 33)) (probe (position 50 21))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 50 45) (end 50 57)) (probe (position 50 45))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::frame"))) (kind expressionOperand) (ordinal 0) (authored-target "defaultFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame")))))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 48 19) (end 48 24)) (probe (position 48 19))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::point"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 52 32) (end 52 48)) (probe (position 52 32))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::positionVector"))) (kind featureTyping) (ordinal 0) (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 49 18) (end 49 32)) (probe (position 49 18))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::PositionOf::time"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 32 45) (end 32 49)) (probe (position 32 45))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::SpatialFrame"))) (kind specialization) (ordinal 0) (authored-target "Body")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_frames.md") (range (start 25 27) (end 25 43)) (probe (position 25 27))
    (reference (id (source (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::defaultFrame"))) (kind featureTyping) (ordinal 0) (authored-target "DefaultFrameLife")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_frames.md") (qualified-name "SpatialFrames::DefaultFrameLife")))))
  )
)
~~~
