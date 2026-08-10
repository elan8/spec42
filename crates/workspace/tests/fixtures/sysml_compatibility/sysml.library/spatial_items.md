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
# EXPECTED
~~~
semantic.unresolved_name 'SpatialFrame'
semantic.unresolved_name 'localClock'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeDCoordinateFrame'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'spaceShots'
semantic.unresolved_name 'subitems'
semantic.unresolved_name 'SpatialItem::localClock'
semantic.unresolved_name 'subitems::localClock'
semantic.unresolved_name 'subparts'
semantic.unresolved_name 'SpatialItem::localClock'
semantic.unresolved_name 'subSpatialItems::localClock'
semantic.unresolved_name 'subparts::localClock'
semantic.unresolved_name 'SpatialItem::localClock'
semantic.unresolved_name 'subSpatialItems::localClock'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'source'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'unionsOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem::localClock'
semantic.unresolved_name 'componentItems::localClock'
semantic.unresolved_name 'subSpatialParts::localClock'
semantic.unresolved_name 'subparts::localClock'
semantic.unresolved_name 'SpatialFrames::PositionOf'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'TimeInstantValue'
semantic.unresolved_name 'frame'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'Position3dVector'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'SpatialFrames::CurrentPositionOf'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'frame'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'Position3dVector'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'SpatialFrames::DisplacementOf'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'TimeInstantValue'
semantic.unresolved_name 'frame'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'Displacement3dVector'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'SpatialFrames::CurrentDisplacementOf'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'frame'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'Displacement3dVector'
semantic.unresolved_name 'mRef'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SpatialFrame'
semantic.unresolved_name 'localClock'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'ThreeDCoordinateFrame'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'spaceShots'
semantic.unresolved_name 'subitems'
semantic.unresolved_name 'SpatialItem::localClock'
semantic.unresolved_name 'subitems::localClock'
semantic.unresolved_name 'subparts'
semantic.unresolved_name 'SpatialItem::localClock'
semantic.unresolved_name 'subSpatialItems::localClock'
semantic.unresolved_name 'subparts::localClock'
semantic.unresolved_name 'SpatialItem::localClock'
semantic.unresolved_name 'subSpatialItems::localClock'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'source'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'unionsOf'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem::localClock'
semantic.unresolved_name 'componentItems::localClock'
semantic.unresolved_name 'subSpatialParts::localClock'
semantic.unresolved_name 'subparts::localClock'
semantic.unresolved_name 'SpatialFrames::PositionOf'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'TimeInstantValue'
semantic.unresolved_name 'frame'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'Position3dVector'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'SpatialFrames::CurrentPositionOf'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'frame'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'Position3dVector'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'SpatialFrames::DisplacementOf'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'TimeInstantValue'
semantic.unresolved_name 'frame'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'Displacement3dVector'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'SpatialFrames::CurrentDisplacementOf'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'Point'
semantic.unresolved_name 'frame'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'Displacement3dVector'
semantic.unresolved_name 'mRef'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAssert,KwConstraint,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,OpenParen,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwRef,KwItem,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwRef,KwItem,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwItem,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,KwDefault,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,KwDefault,OpenParen,Ident,Dot,Ident,KwAs,Ident,CloseParen,Dot,Ident,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,KwDefault,OpenParen,Ident,Dot,Ident,Dot,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwIf,Ident,OpenParen,Ident,CloseParen,Question,DecimalValue,KwElse,DecimalValue,Semicolon,
KwPrivate,KwAttribute,Ident,OpenSquare,Ident,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Eq,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwRef,KwItem,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,ColonGtGt,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,ColonGtGt,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,ColonGtGt,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwCalc,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,ColonGtGt,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'SpatialItems'
    (documentation)
    (import_decl private 'Objects::Point')
    (import_decl private 'SpatialFrames::SpatialFrame')
    (import_decl private 'Quantities::VectorQuantityValue')
    (import_decl private 'MeasurementReferences::ThreeDCoordinateFrame')
    (import_decl private 'MeasurementReferences::nullTransformation')
    (import_decl private 'Time::Clock')
    (import_decl private 'Time::TimeInstantValue')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'ISQ::universalCartesianSpatial3dCoordinateFrame')
    (import_decl private 'ISQ::Position3dVector')
    (import_decl private 'ISQ::Displacement3dVector')
    (import_decl private 'VectorFunctions::isZeroVector')
    (import_decl private 'SequenceFunctions::isEmpty')
    (import_decl private 'ControlFunctions::forAll')
    (item_def 'SpatialItem' :> 'SpatialFrame'
      (documentation)
      (item_usage ref :>> 'localClock' : 'Clock' multiplicity value
        (documentation))
      (attribute_usage 'coordinateFrame' : 'ThreeDCoordinateFrame' multiplicity value
        (documentation))
      (item_usage 'originPoint' : 'Point' :> 'spaceShots' multiplicity
        (documentation))
      (sysml_decl 'originPointConstraint'
        (documentation)
        (result_expr_member))
      (item_usage 'subSpatialItems' : 'SpatialItem' :> 'subitems' multiplicity
        (item_usage ref :>> 'SpatialItem::localClock', 'subitems::localClock'))
      (part_usage 'subSpatialParts' : 'SpatialItem' :> 'subSpatialItems', 'subparts' multiplicity
        (item_usage ref :>> 'SpatialItem::localClock', 'subSpatialItems::localClock', 'subparts::localClock'))
      (item_usage 'componentItems' : 'SpatialItem' :> 'subSpatialItems' multiplicity
        (documentation)
        (item_usage ref :>> 'SpatialItem::localClock', 'subSpatialItems::localClock' value)
        (attribute_usage :>> 'coordinateFrame'
          (attribute_usage :>> 'mRefs' value)
          (attribute_usage :>> 'transformation' multiplicity value
            (attribute_usage :>> 'source' value))))
      (attribute_usage private 'cunionNum' : 'Natural' multiplicity value)
      (attribute_usage private 'componentUnion' :> 'unionsOf' multiplicity
        (documentation)
        (item_usage :>> 'elements' : 'SpatialItem' multiplicity value))
      (part_usage 'componentParts' : 'SpatialItem' :> 'componentItems', 'subSpatialParts' multiplicity
        (item_usage ref :>> 'SpatialItem::localClock', 'componentItems::localClock', 'subSpatialParts::localClock', 'subparts::localClock')))
    (calc_def 'PositionOf' :> 'SpatialFrames::PositionOf'
      (documentation)
      (default_ref_usage in 'point' : 'Point' multiplicity)
      (default_ref_usage in 'timeInstant' : 'TimeInstantValue' multiplicity)
      (default_ref_usage in 'enclosingItem' :>> ''frame'' : 'SpatialItem' multiplicity)
      (default_ref_usage in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (calc_def 'CurrentPositionOf' :> 'SpatialFrames::CurrentPositionOf'
      (documentation)
      (default_ref_usage in 'point' : 'Point' multiplicity)
      (default_ref_usage in 'enclosingItem' :>> ''frame'' : 'SpatialItem' multiplicity)
      (default_ref_usage in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (calc_def 'DisplacementOf' :> 'SpatialFrames::DisplacementOf'
      (documentation)
      (default_ref_usage in 'point1' : 'Point' multiplicity)
      (default_ref_usage in 'point2' : 'Point' multiplicity)
      (default_ref_usage in 'timeInstant' : 'TimeInstantValue' multiplicity)
      (default_ref_usage in 'spatialItem' :>> ''frame'' : 'SpatialItem' multiplicity)
      (default_ref_usage in 'clock' : 'Clock' multiplicity value)
      (return_member))
    (calc_def 'CurrentDisplacementOf' :> 'SpatialFrames::CurrentDisplacementOf'
      (documentation)
      (default_ref_usage in 'point1' : 'Point' multiplicity)
      (default_ref_usage in 'point2' : 'Point' multiplicity)
      (default_ref_usage in 'spatialItem' :>> ''frame'' : 'SpatialItem' multiplicity)
      (default_ref_usage in 'clock' : 'Clock' multiplicity value)
      (return_member))))
~~~
# FORMAT
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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "SpatialItems"))) (name "SpatialItems") (declared-name "SpatialItems")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::Clock"))) (name "Clock") (declared-name "Clock"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (name "CurrentDisplacementOf") (declared-name "CurrentDisplacementOf") (declared (own-expression (expression (kind "featureReference") (reference "displacementVector")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (name "clock") (declared-name "clock") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point1"))) (name "point1") (declared-name "point1") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point2"))) (name "point2") (declared-name "point2") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (name "spatialItem") (declared-name "spatialItem") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (name "CurrentPositionOf") (declared-name "CurrentPositionOf") (declared (own-expression (expression (kind "featureReference") (reference "positionVector")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (name "clock") (declared-name "clock") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (name "enclosingItem") (declared-name "enclosingItem") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::point"))) (name "point") (declared-name "point") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::Displacement3dVector"))) (name "Displacement3dVector") (declared-name "Displacement3dVector"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (name "DisplacementOf") (declared-name "DisplacementOf") (declared (own-expression (expression (kind "featureReference") (reference "displacementVector")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::DisplacementOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::clock"))) (name "clock") (declared-name "clock") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::DisplacementOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point1"))) (name "point1") (declared-name "point1") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::DisplacementOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point2"))) (name "point2") (declared-name "point2") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::DisplacementOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (name "spatialItem") (declared-name "spatialItem") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::DisplacementOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::timeInstant"))) (name "timeInstant") (declared-name "timeInstant") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::DisplacementOf")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::Point"))) (name "Point") (declared-name "Point"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::Position3dVector"))) (name "Position3dVector") (declared-name "Position3dVector"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (name "PositionOf") (declared-name "PositionOf") (declared (own-expression (expression (kind "featureReference") (reference "positionVector")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference")))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::PositionOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::clock"))) (name "clock") (declared-name "clock") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::PositionOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (name "enclosingItem") (declared-name "enclosingItem") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::PositionOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::point"))) (name "point") (declared-name "point") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::PositionOf")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::timeInstant"))) (name "timeInstant") (declared-name "timeInstant") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::PositionOf")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::SpatialFrame"))) (name "SpatialFrame") (declared-name "SpatialFrame"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (name "SpatialItem") (declared-name "SpatialItem")
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::"))) (name "") (declared (properties (composite false) (reference true)) (feature-value (kind default) (expression (kind "featureReference") (reference "Time::universalClock")))) (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (name "componentParts") (declared-name "componentParts") (declared (properties (ordered false)) (multiplicity (lower 1) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (name "componentUnion") (declared-name "componentUnion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (name "cunionNum") (declared-name "cunionNum") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (name "subSpatialParts") (declared-name "subSpatialParts") (declared (properties (ordered false)) (multiplicity (lower 1) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::ThreeDCoordinateFrame"))) (name "ThreeDCoordinateFrame") (declared-name "ThreeDCoordinateFrame"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::TimeInstantValue"))) (name "TimeInstantValue") (declared-name "TimeInstantValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "SpatialItems::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::forAll"))) (name "forAll") (declared-name "forAll"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::isEmpty"))) (name "isEmpty") (declared-name "isEmpty"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::isZeroVector"))) (name "isZeroVector") (declared-name "isZeroVector"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::nullTransformation"))) (name "nullTransformation") (declared-name "nullTransformation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SpatialItems::universalCartesianSpatial3dCoordinateFrame"))) (name "universalCartesianSpatial3dCoordinateFrame") (declared-name "universalCartesianSpatial3dCoordinateFrame"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::_documentation"))) (to (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::_documentation"))) (to (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::_documentation"))) (to (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::PositionOf::_documentation"))) (to (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::_documentation"))) (to (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion::_documentation"))) (to (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame::_documentation"))) (to (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::_documentation"))) (to (node (document "d0") (qualified-name "SpatialItems"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (to (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (to (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (to (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/spatial_items.md"
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
        (range (start 22 1) (end 22 2909))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 28 2) (end 28 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_ref_type_reference")
        (source "semantic")
        (range (start 28 27) (end 28 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 439))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 2) (end 86 83))
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
