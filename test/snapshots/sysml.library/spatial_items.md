# META
~~~ini
description=Standard Library: Domain Libraries/Geometry/SpatialItems
type=file
~~~
# SOURCE
~~~sysml
standard library package SpatialItems {
	doc
	/*
	 * This package models physical items that have a spatial extent and act as a spatial frame of reference
	 * for obtaining position and displacement vectors of points within them.
	 */

	private import Objects::Point;
	private import SpatialFrames::SpatialFrame;
	private import Quantities::VectorQuantityValue;
	private import MeasurementReferences::ThreeDCoordinateFrame;
	private import MeasurementReferences::nullTransformation;
	private import Time::Clock;
	private import Time::TimeInstantValue;
	private import ScalarValues::Natural;
	private import ISQ::universalCartesianSpatial3dCoordinateFrame;
	private import ISQ::Position3dVector;
	private import ISQ::Displacement3dVector;
	private import VectorFunctions::isZeroVector;
	private import SequenceFunctions::isEmpty;
	private import ControlFunctions::forAll;
	
	item def SpatialItem :> SpatialFrame {
		doc
		/*
		 * A SpatialItem is an Item with a three-dimensional spatial extent that also acts as a SpatialFrame of reference.
		 */
	
		ref item :>> localClock : Clock[1] default Time::universalClock {
			doc
			/*
			 * A local Clock to be used as the corresponding time reference within this SpatialItem. 
			 * By default this is the singleton universalClock.
			 */
		}
		
		attribute coordinateFrame : ThreeDCoordinateFrame[1] default universalCartesianSpatial3dCoordinateFrame {
            doc
            /*
             * The three-dimensional CoordinateFrame to be used as the measurement reference for position 
             * and displacement vector values relative to this SpatialItem.
             * By default this is the singleton universalCartesianSpatial3dCoordinateFrame.
             */
        }
        		
		item originPoint : Point[1] :> spaceShots {
			doc
			/*
			 * The Point at the origin of the coordinateFrame of this SpatialItem.
			 */
		}
		
		assert constraint originPointConstraint {
			doc
			/*
			 * The CurrentPositionOf the originPoint must always be a zero vector.
			 */
		
			isZeroVector(CurrentPositionOf(originPoint, that))
		}

        item subSpatialItems : SpatialItem[1..*] :> subitems {
            ref item :>> SpatialItem::localClock, subitems::localClock;
        }
        
        part subSpatialParts : SpatialItem[1..*] :> subSpatialItems, subparts {
            ref item :>> SpatialItem::localClock, subSpatialItems::localClock, subparts::localClock;
        }

		item componentItems : SpatialItem[1..*] :> subSpatialItems {
			doc
			/*
			 * A SpatialItem with componentItems is entirely made up of those items (the SpatialItem occurs only
			 * as a collection of its componentItems).  By default they have the same localClock and equivalent
			 * coordinate frame as the SpatialItem they make up.  A SpatialItem without componentItems occurs
			 * on its own, separately from its subitems.
			 */		
			ref item :>> SpatialItem::localClock, subSpatialItems::localClock default (that as SpatialItem).localClock;
			attribute :>> coordinateFrame {
				attribute :>> mRefs default (that.that as SpatialItem).coordinateFrame.mRefs;
				attribute :>> transformation[1] default nullTransformation {
					attribute :>> source default (that.that.that as SpatialItem).coordinateFrame;
				}
			}
		}

		private attribute cunionNum: Natural [1] = if isEmpty(componentItems) ? 0 else 1;
		private attribute componentUnion[cunionNum] :> unionsOf {
			doc
			/*
			 * A SpatialItem with componentItems is is a spatial union of them.
			 */
		
			item :>> elements : SpatialItem [1..*] = componentItems;
		}
		
		part componentParts : SpatialItem[1..*] :> componentItems, subSpatialParts {
		    ref item :>> SpatialItem::localClock, componentItems::localClock, subSpatialParts::localClock, subparts::localClock;
		}
	}

	calc def PositionOf :> SpatialFrames::PositionOf {
		doc
		/*
		 * The PositionOf a Point relative to a SpatialItem, at a specific TimeInstantValue relative to a given Clock,
		 * is a positionVector that is a VectorQuantityValue in the coordinateFrame of the SpatialItem.
		 * The default Clock is the localClock of the SpatialItem.
		 */
	
		in point : Point[1];
		in timeInstant : TimeInstantValue[1];
		in enclosingItem :>> 'frame' : SpatialItem[1];
		in clock : Clock[1] default enclosingItem.localClock;
		return positionVector : Position3dVector[1] {
			attribute :>> mRef = enclosingItem.coordinateFrame;
		}
	}

	calc def CurrentPositionOf :> SpatialFrames::CurrentPositionOf {
		doc
		/*
		 * The CurrentPositionOf a Point relative to a SpatialItem and a Clock is the PositionOf
		 * the Point relative to the SpatialItem at the currentTime of the Clock.
		 */
	
		in point : Point[1];
		in enclosingItem :>> 'frame' : SpatialItem[1];
		in clock : Clock[1] default enclosingItem.localClock;
		return positionVector : Position3dVector[1] {
			attribute :>> mRef = enclosingItem.coordinateFrame;
		}
	}

	calc def DisplacementOf :> SpatialFrames::DisplacementOf {
		doc
		/*
		 * The DisplacementOf two Points relative to a SpatialItem, at a specific TimeInstantValue relative to a
		 * given Clock, is the displacementVector computed as the difference between the PositionOf the 
		 * first Point and PositionOf the second Point, relative to that SpatialItem, at that timeInstant.
		 */
	
		in point1 : Point[1];
		in point2 : Point[1];
		in timeInstant : TimeInstantValue[1];
		in spatialItem :>> 'frame' : SpatialItem[1];
		in clock : Clock[1] default spatialItem.localClock;
		return displacementVector : Displacement3dVector[1] {
			attribute :>> mRef = spatialItem.coordinateFrame;
		}
	}

	calc def CurrentDisplacementOf :> SpatialFrames::CurrentDisplacementOf {
		doc
		/*
		 * The CurrentDisplacementOf two Points relative to a SpatialItem and a Clock is the DisplacementOf
		 * the Points relative to the SpatialItem, at the currentTime of the Clock.
		 */
	
		in point1 : Point[1];
		in point2 : Point[1];
		in spatialItem :>> 'frame' : SpatialItem[1];
		in clock : Clock[1] default spatialItem.localClock;
		return displacementVector : Displacement3dVector[1] {
			attribute :>> mRef = spatialItem.coordinateFrame;
		}
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/spatial_items.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 16) (end 20 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 25) (end 22 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 15) (end 28 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 28) (end 28 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 30) (end 36 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 63) (end 36 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 21) (end 45 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 3) (end 58 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 47) (end 58 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 62 25) (end 62 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 62 50) (end 62 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 69) (end 65 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 66 25) (end 66 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 66 50) (end 66 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 66 79) (end 66 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 16) (end 77 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 41) (end 77 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 79 18) (end 79 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 79 32) (end 79 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 80 18) (end 80 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 80 44) (end 80 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 19) (end 81 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 81 34) (end 81 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 31) (end 86 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 86 45) (end 86 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 49) (end 87 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 12) (end 93 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 19) (end 97 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 44) (end 97 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 72) (end 97 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 101) (end 97 121))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 101 24) (end 101 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 13) (end 109 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 110 19) (end 110 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 111 23) (end 111 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 13) (end 112 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 112 30) (end 112 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 26) (end 113 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 118 31) (end 118 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 13) (end 125 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 126 23) (end 126 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 13) (end 127 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 127 30) (end 127 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 128 26) (end 128 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 133 28) (end 133 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 141 14) (end 141 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 142 14) (end 142 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 143 19) (end 143 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 144 21) (end 144 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 145 13) (end 145 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 145 30) (end 145 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 146 30) (end 146 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 151 35) (end 151 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 158 14) (end 158 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 159 14) (end 159 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 160 21) (end 160 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 13) (end 161 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 161 30) (end 161 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 162 30) (end 162 50))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:bd57ce9933edb79205cf90aa83c854b8cd85638d43c8dd0680c9e4f74638a197") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::Point") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SpatialFrames::SpatialFrame") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::VectorQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::ThreeDCoordinateFrame") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "MeasurementReferences::nullTransformation") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::Clock") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::TimeInstantValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::universalCartesianSpatial3dCoordinateFrame") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::Position3dVector") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQ::Displacement3dVector") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "VectorFunctions::isZeroVector") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::isEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 13))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialFrames::CurrentDisplacementOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "spatialItem::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::displacementVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Displacement3dVector"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::point1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::point2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem") (direction in)) (redefinition (reference "frame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialFrames::CurrentPositionOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "enclosingItem::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem") (direction in)) (redefinition (reference "frame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::point"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::positionVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Position3dVector"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialFrames::DisplacementOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "spatialItem::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::displacementVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Displacement3dVector"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::point1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::point2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem") (direction in)) (redefinition (reference "frame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::timeInstant"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeInstantValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialFrames::PositionOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (memberAccessOperand (reference "enclosingItem::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem") (direction in)) (redefinition (reference "frame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::point"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::positionVector"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Position3dVector"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::timeInstant"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeInstantValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock")) (redefinition (reference "localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "SpatialItem::localClock")) (redefinition (reference "subSpatialItems::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mRefs"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transformation")) (expressionOperand (reference "nullTransformation"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "source"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem")) (subsetting (reference "componentItems")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "SpatialItem::localClock")) (redefinition (reference "componentItems::localClock")) (redefinition (reference "subSpatialParts::localClock")) (redefinition (reference "subparts::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (subsetting (reference "unionsOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThreeDCoordinateFrame")) (expressionOperand (reference "universalCartesianSpatial3dCoordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPoint"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "originPoint")) (expressionOperand (reference "that")) (invocationCallee (reference "isZeroVector")) (invocationCallee (reference "CurrentPositionOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "SpatialItem::localClock")) (redefinition (reference "subitems::localClock"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem")) (subsetting (reference "subSpatialItems")) (subsetting (reference "subparts"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "SpatialItem::localClock")) (redefinition (reference "subSpatialItems::localClock")) (redefinition (reference "subparts::localClock"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SpatialFrames::SpatialFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::ThreeDCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "MeasurementReferences::nullTransformation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::universalCartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::Position3dVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::Displacement3dVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "VectorFunctions::isZeroVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialFrames::CurrentDisplacementOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "spatialItem::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "Displacement3dVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::point1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::point2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind redefinition) (ordinal 0))
      (authored-target "frame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialFrames::CurrentPositionOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "enclosingItem::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind redefinition) (ordinal 0))
      (authored-target "frame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::point"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::positionVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "Position3dVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialFrames::DisplacementOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "spatialItem::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "Displacement3dVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::point1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::point2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind redefinition) (ordinal 0))
      (authored-target "frame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::timeInstant"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeInstantValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialFrames::PositionOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::clock"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "enclosingItem::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind redefinition) (ordinal 0))
      (authored-target "frame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::point"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::positionVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "Position3dVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::timeInstant"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeInstantValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "SpatialItem::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1))
      (authored-target "subSpatialItems::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mRefs")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "nullTransformation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 0))
      (authored-target "componentItems")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 1))
      (authored-target "subSpatialParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "SpatialItem::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1))
      (authored-target "componentItems::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 2))
      (authored-target "subSpatialParts::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 3))
      (authored-target "subparts::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (kind subsetting) (ordinal 0))
      (authored-target "unionsOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThreeDCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind expressionOperand) (ordinal 0))
      (authored-target "universalCartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind expressionOperand) (ordinal 0))
      (authored-target "originPoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPoint")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind expressionOperand) (ordinal 1))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind invocationCallee) (ordinal 0))
      (authored-target "isZeroVector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind invocationCallee) (ordinal 1))
      (authored-target "CurrentPositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "SpatialItem::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1))
      (authored-target "subitems::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialItems")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 1))
      (authored-target "subparts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "SpatialItem::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1))
      (authored-target "subSpatialItems::localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 2))
      (authored-target "subparts::localClock")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind invocationCallee) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/spatial_items.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 8 16) (end 8 43)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SpatialFrames::SpatialFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 9 16) (end 9 47)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 10 16) (end 10 60)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::ThreeDCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 11 16) (end 11 57)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::nullTransformation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 12 16) (end 12 27)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Time::Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 13 16) (end 13 38)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 14 16) (end 14 37)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 15 16) (end 15 63)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::universalCartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 16 16) (end 16 37)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::Position3dVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 17 16) (end 17 41)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::Displacement3dVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 18 16) (end 18 45)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "VectorFunctions::isZeroVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 19 16) (end 19 42)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 20 16) (end 20 40)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 151 35) (end 151 71)) (probe (position 151 35))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (kind specialization) (ordinal 0) (authored-target "SpatialFrames::CurrentDisplacementOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 161 13) (end 161 18)) (probe (position 161 13))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 161 30) (end 161 52)) (probe (position 161 30))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "spatialItem::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 162 30) (end 162 50)) (probe (position 162 30))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0) (authored-target "Displacement3dVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 158 14) (end 158 19)) (probe (position 158 14))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::point1"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 159 14) (end 159 19)) (probe (position 159 14))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::point2"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 160 31) (end 160 42)) (probe (position 160 31))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 160 21) (end 160 28)) (probe (position 160 21))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind redefinition) (ordinal 0) (authored-target "frame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 118 31) (end 118 63)) (probe (position 118 31))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf"))) (kind specialization) (ordinal 0) (authored-target "SpatialFrames::CurrentPositionOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 127 13) (end 127 18)) (probe (position 127 13))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 127 30) (end 127 54)) (probe (position 127 30))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "enclosingItem::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 126 33) (end 126 44)) (probe (position 126 33))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 126 23) (end 126 30)) (probe (position 126 23))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind redefinition) (ordinal 0) (authored-target "frame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 125 13) (end 125 18)) (probe (position 125 13))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::point"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 128 26) (end 128 42)) (probe (position 128 26))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf::positionVector"))) (kind featureTyping) (ordinal 0) (authored-target "Position3dVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 133 28) (end 133 57)) (probe (position 133 28))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf"))) (kind specialization) (ordinal 0) (authored-target "SpatialFrames::DisplacementOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 145 13) (end 145 18)) (probe (position 145 13))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 145 30) (end 145 52)) (probe (position 145 30))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "spatialItem::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 146 30) (end 146 50)) (probe (position 146 30))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::displacementVector"))) (kind featureTyping) (ordinal 0) (authored-target "Displacement3dVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 141 14) (end 141 19)) (probe (position 141 14))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::point1"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 142 14) (end 142 19)) (probe (position 142 14))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::point2"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 144 31) (end 144 42)) (probe (position 144 31))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 144 21) (end 144 28)) (probe (position 144 21))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind redefinition) (ordinal 0) (authored-target "frame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 143 19) (end 143 35)) (probe (position 143 19))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::DisplacementOf::timeInstant"))) (kind featureTyping) (ordinal 0) (authored-target "TimeInstantValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 101 24) (end 101 49)) (probe (position 101 24))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf"))) (kind specialization) (ordinal 0) (authored-target "SpatialFrames::PositionOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 112 13) (end 112 18)) (probe (position 112 13))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 112 30) (end 112 54)) (probe (position 112 30))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::clock"))) (kind memberAccessOperand) (ordinal 0) (authored-target "enclosingItem::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 111 33) (end 111 44)) (probe (position 111 33))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 111 23) (end 111 30)) (probe (position 111 23))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind redefinition) (ordinal 0) (authored-target "frame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 109 13) (end 109 18)) (probe (position 109 13))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::point"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 113 26) (end 113 42)) (probe (position 113 26))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::positionVector"))) (kind featureTyping) (ordinal 0) (authored-target "Position3dVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 110 19) (end 110 35)) (probe (position 110 19))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::PositionOf::timeInstant"))) (kind featureTyping) (ordinal 0) (authored-target "TimeInstantValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 22 25) (end 22 37)) (probe (position 22 25))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (kind specialization) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 28 28) (end 28 33)) (probe (position 28 28))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 28 15) (end 28 25)) (probe (position 28 15))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 69 24) (end 69 35)) (probe (position 69 24))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 77 16) (end 77 39)) (probe (position 77 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "SpatialItem::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 78 17) (end 78 32)) (probe (position 78 17))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 77 41) (end 77 68)) (probe (position 77 41))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1) (authored-target "subSpatialItems::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 79 18) (end 79 23)) (probe (position 79 18))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mRefs")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 80 18) (end 80 32)) (probe (position 80 18))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 80 44) (end 80 62)) (probe (position 80 44))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "nullTransformation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 81 19) (end 81 25)) (probe (position 81 19))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 96 24) (end 96 35)) (probe (position 96 24))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 96 45) (end 96 59)) (probe (position 96 45))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 0) (authored-target "componentItems")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 96 61) (end 96 76)) (probe (position 96 61))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 1) (authored-target "subSpatialParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 97 19) (end 97 42)) (probe (position 97 19))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "SpatialItem::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 97 44) (end 97 70)) (probe (position 97 44))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1) (authored-target "componentItems::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 97 72) (end 97 99)) (probe (position 97 72))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 2) (authored-target "subSpatialParts::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 97 101) (end 97 121)) (probe (position 97 101))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 3) (authored-target "subparts::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 87 49) (end 87 57)) (probe (position 87 49))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (kind subsetting) (ordinal 0) (authored-target "unionsOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 93 23) (end 93 34)) (probe (position 93 23))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 93 12) (end 93 20)) (probe (position 93 12))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 36 30) (end 36 51)) (probe (position 36 30))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind featureTyping) (ordinal 0) (authored-target "ThreeDCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 36 63) (end 36 105)) (probe (position 36 63))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind expressionOperand) (ordinal 0) (authored-target "universalCartesianSpatial3dCoordinateFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 86 31) (end 86 38)) (probe (position 86 31))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 45 21) (end 45 26)) (probe (position 45 21))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPoint"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 58 34) (end 58 45)) (probe (position 58 34))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind expressionOperand) (ordinal 0) (authored-target "originPoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPoint")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 58 47) (end 58 51)) (probe (position 58 47))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind expressionOperand) (ordinal 1) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 58 3) (end 58 15)) (probe (position 58 3))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind invocationCallee) (ordinal 0) (authored-target "isZeroVector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 58 16) (end 58 33)) (probe (position 58 16))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPointConstraint"))) (kind invocationCallee) (ordinal 1) (authored-target "CurrentPositionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::CurrentPositionOf")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 61 31) (end 61 42)) (probe (position 61 31))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 62 25) (end 62 48)) (probe (position 62 25))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "SpatialItem::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 62 50) (end 62 70)) (probe (position 62 50))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1) (authored-target "subitems::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 65 31) (end 65 42)) (probe (position 65 31))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 65 52) (end 65 67)) (probe (position 65 52))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 0) (authored-target "subSpatialItems")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 65 69) (end 65 77)) (probe (position 65 69))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 1) (authored-target "subparts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 66 25) (end 66 48)) (probe (position 66 25))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "SpatialItem::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 66 50) (end 66 77)) (probe (position 66 50))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 1) (authored-target "subSpatialItems::localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 66 79) (end 66 99)) (probe (position 66 79))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 2) (authored-target "subparts::localClock")
      (outcome (status unresolved)))
  )
)
~~~
