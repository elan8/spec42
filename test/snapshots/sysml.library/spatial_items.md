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
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 28 2) (end 34 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 30) (end 36 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 21) (end 45 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 52 2) (end 59 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 62 12) (end 62 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 69) (end 65 77))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 66 12) (end 67 8))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 66 12) (end 67 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 77 3) (end 77 110))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 79 18) (end 79 23))
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
        (range (start 81 19) (end 81 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 31) (end 86 38))
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 101 1) (end 116 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 118 1) (end 131 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 133 1) (end 149 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 151 1) (end 165 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:bd57ce9933edb79205cf90aa83c854b8cd85638d43c8dd0680c9e4f74638a197") (contract-version "parser-owned-resolution-v1"))
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
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpatialFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "coordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mRefs"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transformation"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "source"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem")) (subsetting (reference "componentItems")) (subsetting (reference "subSpatialParts"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (subsetting (reference "unionsOf"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThreeDCoordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPoint"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Point"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem"))))
    (declaration (id (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpatialItem")) (subsetting (reference "subSpatialItems")) (subsetting (reference "subparts"))))
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
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (kind specialization) (ordinal 0))
      (authored-target "SpatialFrame")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "coordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mRefs")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "transformation")
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
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "Point")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 0))
      (authored-target "subSpatialItems")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems")))))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 1))
      (authored-target "subparts")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
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
  (query (document "memory://snapshot/spatial_items.md") (range (start 22 25) (end 22 37)) (probe (position 22 25))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem"))) (kind specialization) (ordinal 0) (authored-target "SpatialFrame")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 69 24) (end 69 35)) (probe (position 69 24))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::componentItems"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 78 17) (end 78 32)) (probe (position 78 17))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "coordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::coordinateFrame")))))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 79 18) (end 79 23)) (probe (position 79 18))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mRefs")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 80 18) (end 80 32)) (probe (position 80 18))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "transformation")
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
  (query (document "memory://snapshot/spatial_items.md") (range (start 86 31) (end 86 38)) (probe (position 86 31))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 45 21) (end 45 26)) (probe (position 45 21))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::originPoint"))) (kind featureTyping) (ordinal 0) (authored-target "Point")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/spatial_items.md") (range (start 61 31) (end 61 42)) (probe (position 61 31))
    (reference (id (source (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem::subSpatialItems"))) (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/spatial_items.md") (qualified-name "SpatialItems::SpatialItem")))))
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
)
~~~
