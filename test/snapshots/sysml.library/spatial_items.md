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
  (document "spatial_items.md"
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 15) (end 28 25))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 36 2) (end 36 439))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 52) (end 65 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 69) (end 65 77))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 86 2) (end 86 83))
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
        (range (start 96 45) (end 96 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 2) (end 109 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 110 2) (end 110 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 2) (end 111 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 2) (end 112 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 2) (end 125 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 126 2) (end 126 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 2) (end 127 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 141 2) (end 141 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 142 2) (end 142 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 143 2) (end 143 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 144 2) (end 144 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 145 2) (end 145 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 158 2) (end 158 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 159 2) (end 159 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 160 2) (end 160 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 2) (end 161 53))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "9edceafffa3f17f93b2ccba4fa5d253e3c4909fa43666f7f4a9559d37fb1865f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SpatialItems"))) (kind "package") (name "SpatialItems") (declared-name "SpatialItems"))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Clock"))) (kind "import") (name "Clock") (declared-name "Clock") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::Clock") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (kind "calc def") (name "CurrentDisplacementOf") (declared-name "CurrentDisplacementOf") (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (authored (relationships (typing (reference "clock : Clock[1] default spatialItem.localClock")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point1"))) (kind "in out parameter") (name "point1") (declared-name "point1") (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (authored (relationships (typing (reference "point1 : Point[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point2"))) (kind "in out parameter") (name "point2") (declared-name "point2") (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (authored (relationships (typing (reference "point2 : Point[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind "in out parameter") (name "spatialItem") (declared-name "spatialItem") (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (authored (relationships (typing (reference "spatialItem :>> 'frame' : SpatialItem[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (kind "calc def") (name "CurrentPositionOf") (declared-name "CurrentPositionOf") (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (parent (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (authored (relationships (typing (reference "clock : Clock[1] default enclosingItem.localClock")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind "in out parameter") (name "enclosingItem") (declared-name "enclosingItem") (parent (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (authored (relationships (typing (reference "enclosingItem :>> 'frame' : SpatialItem[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::point"))) (kind "in out parameter") (name "point") (declared-name "point") (parent (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (authored (relationships (typing (reference "point : Point[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Displacement3dVector"))) (kind "import") (name "Displacement3dVector") (declared-name "Displacement3dVector") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::Displacement3dVector") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (kind "calc def") (name "DisplacementOf") (declared-name "DisplacementOf") (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "clock : Clock[1] default spatialItem.localClock")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point1"))) (kind "in out parameter") (name "point1") (declared-name "point1") (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "point1 : Point[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point2"))) (kind "in out parameter") (name "point2") (declared-name "point2") (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "point2 : Point[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind "in out parameter") (name "spatialItem") (declared-name "spatialItem") (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "spatialItem :>> 'frame' : SpatialItem[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::timeInstant"))) (kind "in out parameter") (name "timeInstant") (declared-name "timeInstant") (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "timeInstant : TimeInstantValue[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Point"))) (kind "import") (name "Point") (declared-name "Point") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Point") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Position3dVector"))) (kind "import") (name "Position3dVector") (declared-name "Position3dVector") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::Position3dVector") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (kind "calc def") (name "PositionOf") (declared-name "PositionOf") (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (authored (relationships (typing (reference "clock : Clock[1] default enclosingItem.localClock")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind "in out parameter") (name "enclosingItem") (declared-name "enclosingItem") (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (authored (relationships (typing (reference "enclosingItem :>> 'frame' : SpatialItem[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::point"))) (kind "in out parameter") (name "point") (declared-name "point") (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (authored (relationships (typing (reference "point : Point[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::timeInstant"))) (kind "in out parameter") (name "timeInstant") (declared-name "timeInstant") (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (authored (relationships (typing (reference "timeInstant : TimeInstantValue[1]")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialFrame"))) (kind "import") (name "SpatialFrame") (declared-name "SpatialFrame") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SpatialFrames::SpatialFrame") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (kind "item def") (name "SpatialItem") (declared-name "SpatialItem") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialFrame")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::"))) (kind "ref") (name "") (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature)) (relationships (typing (reference "Clock")) (redefinition (reference "localClock")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind "part") (name "componentParts") (declared-name "componentParts") (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpatialItem")) (subsetting (reference "componentItems")) (subsetting (reference "subSpatialParts")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (kind "attribute") (name "componentUnion") (declared-name "componentUnion") (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature) (visibility "private")) (relationships (subsetting (reference "unionsOf")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThreeDCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind "attribute") (name "cunionNum") (declared-name "cunionNum") (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind "part") (name "subSpatialParts") (declared-name "subSpatialParts") (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpatialItem")) (subsetting (reference "subSpatialItems")) (subsetting (reference "subparts")))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::ThreeDCoordinateFrame"))) (kind "import") (name "ThreeDCoordinateFrame") (declared-name "ThreeDCoordinateFrame") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::ThreeDCoordinateFrame") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::TimeInstantValue"))) (kind "import") (name "TimeInstantValue") (declared-name "TimeInstantValue") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::TimeInstantValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::isZeroVector"))) (kind "import") (name "isZeroVector") (declared-name "isZeroVector") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorFunctions::isZeroVector") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::nullTransformation"))) (kind "import") (name "nullTransformation") (declared-name "nullTransformation") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::nullTransformation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::universalCartesianSpatial3dCoordinateFrame"))) (kind "import") (name "universalCartesianSpatial3dCoordinateFrame") (declared-name "universalCartesianSpatial3dCoordinateFrame") (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::universalCartesianSpatial3dCoordinateFrame") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Clock"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::Clock") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default spatialItem.localClock") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point1"))) (kind featureTyping) (ordinal 0)) (authored-target "point1 : Point[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point2"))) (kind featureTyping) (ordinal 0)) (authored-target "point2 : Point[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0)) (authored-target "spatialItem :>> 'frame' : SpatialItem[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default enclosingItem.localClock") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0)) (authored-target "enclosingItem :>> 'frame' : SpatialItem[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::point"))) (kind featureTyping) (ordinal 0)) (authored-target "point : Point[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Displacement3dVector"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::Displacement3dVector") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default spatialItem.localClock") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point1"))) (kind featureTyping) (ordinal 0)) (authored-target "point1 : Point[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point2"))) (kind featureTyping) (ordinal 0)) (authored-target "point2 : Point[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0)) (authored-target "spatialItem :>> 'frame' : SpatialItem[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::timeInstant"))) (kind featureTyping) (ordinal 0)) (authored-target "timeInstant : TimeInstantValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Point"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Point") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Position3dVector"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::Position3dVector") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::PositionOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default enclosingItem.localClock") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0)) (authored-target "enclosingItem :>> 'frame' : SpatialItem[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::PositionOf::point"))) (kind featureTyping) (ordinal 0)) (authored-target "point : Point[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::PositionOf::timeInstant"))) (kind featureTyping) (ordinal 0)) (authored-target "timeInstant : TimeInstantValue[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "SpatialFrames::SpatialFrame") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (kind specialization) (ordinal 0)) (authored-target "SpatialFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::SpatialFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::"))) (kind featureTyping) (ordinal 0)) (authored-target "Clock") (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::Clock")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::"))) (kind redefinition) (ordinal 0)) (authored-target "localClock") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind featureTyping) (ordinal 0)) (authored-target "SpatialItem") (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 0)) (authored-target "componentItems") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 1)) (authored-target "subSpatialParts") (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (kind subsetting) (ordinal 0)) (authored-target "unionsOf") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "ThreeDCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::ThreeDCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind featureTyping) (ordinal 0)) (authored-target "SpatialItem") (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialItems") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 1)) (authored-target "subparts") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::ThreeDCoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::ThreeDCoordinateFrame") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::TimeInstantValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::TimeInstantValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::isZeroVector"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorFunctions::isZeroVector") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::nullTransformation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::nullTransformation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::universalCartesianSpatial3dCoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::universalCartesianSpatial3dCoordinateFrame") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (target (node (document "d0") (qualified-name "SpatialItems::SpatialFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::"))) (target (node (document "d0") (qualified-name "SpatialItems::Clock"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (target (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (target (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (target (node (document "d0") (qualified-name "SpatialItems::ThreeDCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (target (node (document "d0") (qualified-name "SpatialItems::Natural"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (target (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SpatialItems::DisplacementOf")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SpatialItems::PositionOf")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SpatialItems::SpatialItem::")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 28 27) (end 28 33)) (probe (position 28 27))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem::"))
        (kind featureTyping) (ordinal 0) (authored-target "Clock")
        (range (start 28 27) (end 28 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SpatialItems::Clock") (range (start 12 1) (end 12 28)))
        )
      )
    )
    (query (range (start 65 69) (end 65 77)) (probe (position 65 69))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))
        (kind subsetting) (ordinal 1) (authored-target "subparts")
        (range (start 65 69) (end 65 77))
        (outcome (status unresolved))
      )
    )
    (query (range (start 87 49) (end 87 57)) (probe (position 87 49))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))
        (kind subsetting) (ordinal 0) (authored-target "unionsOf")
        (range (start 87 49) (end 87 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 15) (end 28 25)) (probe (position 28 15))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem::"))
        (kind redefinition) (ordinal 0) (authored-target "localClock")
        (range (start 28 15) (end 28 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 27)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::Clock"))
        (kind membershipImport) (ordinal 0) (authored-target "Time::Clock")
        (range (start 12 16) (end 12 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 65 31) (end 65 42)) (probe (position 65 31))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))
        (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
        (range (start 65 31) (end 65 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SpatialItems::SpatialItem") (range (start 22 1) (end 22 2909)))
        )
      )
    )
    (query (range (start 96 24) (end 96 35)) (probe (position 96 24))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))
        (kind featureTyping) (ordinal 0) (authored-target "SpatialItem")
        (range (start 96 24) (end 96 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SpatialItems::SpatialItem") (range (start 22 1) (end 22 2909)))
        )
      )
    )
    (query (range (start 22 25) (end 22 37)) (probe (position 22 25))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem"))
        (kind specialization) (ordinal 0) (authored-target "SpatialFrame")
        (range (start 22 25) (end 22 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SpatialItems::SpatialFrame") (range (start 8 1) (end 8 44)))
        )
      )
    )
    (query (range (start 7 16) (end 7 30)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::Point"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::Point")
        (range (start 7 16) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 96 45) (end 96 59)) (probe (position 96 45))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))
        (kind subsetting) (ordinal 0) (authored-target "componentItems")
        (range (start 96 45) (end 96 59))
        (outcome (status unresolved))
      )
    )
    (query (range (start 65 52) (end 65 67)) (probe (position 65 52))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))
        (kind subsetting) (ordinal 0) (authored-target "subSpatialItems")
        (range (start 65 52) (end 65 67))
        (outcome (status unresolved))
      )
    )
    (query (range (start 96 61) (end 96 76)) (probe (position 96 61))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))
        (kind subsetting) (ordinal 1) (authored-target "subSpatialParts")
        (range (start 96 61) (end 96 76))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts") (range (start 65 8) (end 65 190)))
        )
      )
    )
    (query (range (start 14 16) (end 14 37)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 14 16) (end 14 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 37)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::Position3dVector"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::Position3dVector")
        (range (start 16 16) (end 16 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 38)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::TimeInstantValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Time::TimeInstantValue")
        (range (start 13 16) (end 13 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 16) (end 20 40)) (probe (position 20 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 20 16) (end 20 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 41)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::Displacement3dVector"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::Displacement3dVector")
        (range (start 17 16) (end 17 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 16) (end 19 42)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::isEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
        (range (start 19 16) (end 19 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 43)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::SpatialFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "SpatialFrames::SpatialFrame")
        (range (start 8 16) (end 8 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 45)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::isZeroVector"))
        (kind membershipImport) (ordinal 0) (authored-target "VectorFunctions::isZeroVector")
        (range (start 18 16) (end 18 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 47)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::VectorQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
        (range (start 9 16) (end 9 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 57)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::nullTransformation"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::nullTransformation")
        (range (start 11 16) (end 11 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 60)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::ThreeDCoordinateFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "MeasurementReferences::ThreeDCoordinateFrame")
        (range (start 10 16) (end 10 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 63)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "SpatialItems::universalCartesianSpatial3dCoordinateFrame"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::universalCartesianSpatial3dCoordinateFrame")
        (range (start 15 16) (end 15 63))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
