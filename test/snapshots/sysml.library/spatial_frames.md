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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4132489f2c2297db203fcc5585e68ed65bc1ba3b1786e495c4cbe8638b1b87e8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SpatialFrames"))) (kind "package") (name "SpatialFrames") (declared-name "SpatialFrames"))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Clocks::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::Body"))) (kind "import") (name "Body") (declared-name "Body") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Body") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianCurrentDisplacementOf"))) (kind "kermlDecl") (name "CartesianCurrentDisplacementOf") (declared-name "CartesianCurrentDisplacementOf") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianCurrentPositionOf"))) (kind "kermlDecl") (name "CartesianCurrentPositionOf") (declared-name "CartesianCurrentPositionOf") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianDisplacementOf"))) (kind "kermlDecl") (name "CartesianDisplacementOf") (declared-name "CartesianDisplacementOf") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianPositionOf"))) (kind "kermlDecl") (name "CartesianPositionOf") (declared-name "CartesianPositionOf") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianSpatialFrame"))) (kind "classifier decl") (name "CartesianSpatialFrame") (declared-name "CartesianSpatialFrame") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CartesianThreeVectorValue"))) (kind "import") (name "CartesianThreeVectorValue") (declared-name "CartesianThreeVectorValue") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorValues::CartesianThreeVectorValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CurrentDisplacementOf"))) (kind "kermlDecl") (name "CurrentDisplacementOf") (declared-name "CurrentDisplacementOf") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::CurrentPositionOf"))) (kind "kermlDecl") (name "CurrentPositionOf") (declared-name "CurrentPositionOf") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::DefaultFrameLife1"))) (kind "classifier decl") (name "DefaultFrameLife1") (declared-name "DefaultFrameLife1") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::DisplacementOf"))) (kind "kermlDecl") (name "DisplacementOf") (declared-name "DisplacementOf") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::Life"))) (kind "import") (name "Life") (declared-name "Life") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Life") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::NumericalValue"))) (kind "import") (name "NumericalValue") (declared-name "NumericalValue") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::NumericalValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::Point"))) (kind "import") (name "Point") (declared-name "Point") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Point") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::PositionOf"))) (kind "kermlDecl") (name "PositionOf") (declared-name "PositionOf") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::SpatialFrame"))) (kind "classifier decl") (name "SpatialFrame") (declared-name "SpatialFrame") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::ThreeVectorValue"))) (kind "import") (name "ThreeVectorValue") (declared-name "ThreeVectorValue") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorValues::ThreeVectorValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::defaultFrame"))) (kind "feature decl") (name "defaultFrame") (declared-name "defaultFrame") (parent (node (document "d0") (qualified-name "SpatialFrames"))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::includes"))) (kind "import") (name "includes") (declared-name "includes") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialFrames::isZeroVector"))) (kind "import") (name "isZeroVector") (declared-name "isZeroVector") (parent (node (document "d0") (qualified-name "SpatialFrames"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorFunctions::isZeroVector") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Clocks::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::Body"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Body") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::CartesianThreeVectorValue"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorValues::CartesianThreeVectorValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::Life"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Life") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::NumericalValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::NumericalValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::Point"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Point") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::ThreeVectorValue"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorValues::ThreeVectorValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialFrames::isZeroVector"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorFunctions::isZeroVector") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 19) (end 7 25)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Clocks::*")
        (range (start 7 19) (end 7 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 19) (end 13 32)) (probe (position 13 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::Body"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::Body")
        (range (start 13 19) (end 13 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 19) (end 14 33)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::Point"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::Point")
        (range (start 14 19) (end 14 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 19) (end 12 36)) (probe (position 12 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::Life"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Life")
        (range (start 12 19) (end 12 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 19) (end 15 43)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 15 19) (end 15 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 19) (end 16 46)) (probe (position 16 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::includes"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::includes")
        (range (start 16 19) (end 16 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 19) (end 8 47)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::NumericalValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::NumericalValue")
        (range (start 8 19) (end 8 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 19) (end 11 48)) (probe (position 11 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::isZeroVector"))
        (kind membershipImport) (ordinal 0) (authored-target "VectorFunctions::isZeroVector")
        (range (start 11 19) (end 11 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 49)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::ThreeVectorValue"))
        (kind membershipImport) (ordinal 0) (authored-target "VectorValues::ThreeVectorValue")
        (range (start 9 19) (end 9 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 19) (end 10 58)) (probe (position 10 19))
      (reference
        (source (document "d0") (qualified-name "SpatialFrames::CartesianThreeVectorValue"))
        (kind membershipImport) (ordinal 0) (authored-target "VectorValues::CartesianThreeVectorValue")
        (range (start 10 19) (end 10 58))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
