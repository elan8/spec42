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
    doc /*
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
        doc /*
		 * A SpatialItem is an Item with a three-dimensional spatial extent that also acts as a SpatialFrame of reference.
		 */

        ref item :>> localClock : Clock [1] default = Time::universalClock {
            doc /*
			 * A local Clock to be used as the corresponding time reference within this SpatialItem. 
			 * By default this is the singleton universalClock.
			 */
        }

        attribute coordinateFrame : ThreeDCoordinateFrame [1] default = universalCartesianSpatial3dCoordinateFrame {
            doc /*
             * The three-dimensional CoordinateFrame to be used as the measurement reference for position 
             * and displacement vector values relative to this SpatialItem.
             * By default this is the singleton universalCartesianSpatial3dCoordinateFrame.
             */
        }

        item originPoint : Point :> spaceShots [1] {
            doc /*
			 * The Point at the origin of the coordinateFrame of this SpatialItem.
			 */
        }

        assert constraint originPointConstraint {
            doc /*
			 * The CurrentPositionOf the originPoint must always be a zero vector.
			 */

            = isZeroVector(CurrentPositionOf(originPoint, that));
        }

        item subSpatialItems : SpatialItem :> subitems [1..*] {
            ref item :>> SpatialItem::localClock, subitems::localClock;
        }

        part subSpatialParts : SpatialItem :> subSpatialItems, subparts [1..*] {
            ref item :>> SpatialItem::localClock, subSpatialItems::localClock, subparts::localClock;
        }

        item componentItems : SpatialItem :> subSpatialItems [1..*] {
            doc /*
			 * A SpatialItem with componentItems is entirely made up of those items (the SpatialItem occurs only
			 * as a collection of its componentItems).  By default they have the same localClock and equivalent
			 * coordinate frame as the SpatialItem they make up.  A SpatialItem without componentItems occurs
			 * on its own, separately from its subitems.
			 */
            ref item :>> SpatialItem::localClock, subSpatialItems::localClock default = (that as SpatialItem).localClock;
            attribute :>> coordinateFrame {
                attribute :>> mRefs default = (that.that as SpatialItem).coordinateFrame.mRefs;
                attribute :>> transformation [1] default = nullTransformation {
                    attribute :>> source default = (that.that.that as SpatialItem).coordinateFrame;
                }
            }
        }

        private attribute cunionNum : Natural [1] = if isEmpty(componentItems) ? 0 else 1;
        private attribute componentUnion :> unionsOf [cunionNum] {
            doc /*
			 * A SpatialItem with componentItems is is a spatial union of them.
			 */

            item :>> elements : SpatialItem [1..*] = componentItems;
        }

        part componentParts : SpatialItem :> componentItems, subSpatialParts [1..*] {
            ref item :>> SpatialItem::localClock, componentItems::localClock, subSpatialParts::localClock, subparts::localClock;
        }
    }

    calc def PositionOf :> SpatialFrames::PositionOf {
        doc /*
		 * The PositionOf a Point relative to a SpatialItem, at a specific TimeInstantValue relative to a given Clock,
		 * is a positionVector that is a VectorQuantityValue in the coordinateFrame of the SpatialItem.
		 * The default Clock is the localClock of the SpatialItem.
		 */

        in point : Point [1];
        in timeInstant : TimeInstantValue [1];
        in enclosingItem :>> 'frame' : SpatialItem [1];
        in clock : Clock [1] default = enclosingItem.localClock;
        return positionVector : Position3dVector[1] {
			attribute :>> mRef = enclosingItem.coordinateFrame;
		}
    }

    calc def CurrentPositionOf :> SpatialFrames::CurrentPositionOf {
        doc /*
		 * The CurrentPositionOf a Point relative to a SpatialItem and a Clock is the PositionOf
		 * the Point relative to the SpatialItem at the currentTime of the Clock.
		 */

        in point : Point [1];
        in enclosingItem :>> 'frame' : SpatialItem [1];
        in clock : Clock [1] default = enclosingItem.localClock;
        return positionVector : Position3dVector[1] {
			attribute :>> mRef = enclosingItem.coordinateFrame;
		}
    }

    calc def DisplacementOf :> SpatialFrames::DisplacementOf {
        doc /*
		 * The DisplacementOf two Points relative to a SpatialItem, at a specific TimeInstantValue relative to a
		 * given Clock, is the displacementVector computed as the difference between the PositionOf the 
		 * first Point and PositionOf the second Point, relative to that SpatialItem, at that timeInstant.
		 */

        in point1 : Point [1];
        in point2 : Point [1];
        in timeInstant : TimeInstantValue [1];
        in spatialItem :>> 'frame' : SpatialItem [1];
        in clock : Clock [1] default = spatialItem.localClock;
        return displacementVector : Displacement3dVector[1] {
			attribute :>> mRef = spatialItem.coordinateFrame;
		}
    }

    calc def CurrentDisplacementOf :> SpatialFrames::CurrentDisplacementOf {
        doc /*
		 * The CurrentDisplacementOf two Points relative to a SpatialItem and a Clock is the DisplacementOf
		 * the Points relative to the SpatialItem, at the currentTime of the Clock.
		 */

        in point1 : Point [1];
        in point2 : Point [1];
        in spatialItem :>> 'frame' : SpatialItem [1];
        in clock : Clock [1] default = spatialItem.localClock;
        return displacementVector : Displacement3dVector[1] {
			attribute :>> mRef = spatialItem.coordinateFrame;
		}
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'SpatialItems'
      (documentation)
      (membership_import private -> 'Objects::Point'[unresolved])
      (membership_import private -> 'SpatialFrames::SpatialFrame'[unresolved])
      (membership_import private -> 'Quantities::VectorQuantityValue'[unresolved])
      (membership_import private -> 'MeasurementReferences::ThreeDCoordinateFrame'[unresolved])
      (membership_import private -> 'MeasurementReferences::nullTransformation'[unresolved])
      (membership_import private -> 'Time::Clock'[unresolved])
      (membership_import private -> 'Time::TimeInstantValue'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (membership_import private -> 'ISQ::universalCartesianSpatial3dCoordinateFrame'[unresolved])
      (membership_import private -> 'ISQ::Position3dVector'[unresolved])
      (membership_import private -> 'ISQ::Displacement3dVector'[unresolved])
      (membership_import private -> 'VectorFunctions::isZeroVector'[unresolved])
      (membership_import private -> 'SequenceFunctions::isEmpty'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (item_def 'SpatialItem' :> 'SpatialFrame'[unresolved]
        (documentation)
        (item_usage reference :>> 'localClock'[unresolved] : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation))
        (attribute_usage composite 'coordinateFrame' : 'ThreeDCoordinateFrame'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation))
        (item_usage composite 'originPoint' : 'Point'[unresolved] :> 'spaceShots'[unresolved]
          (multiplicity_range [1])
          (documentation))
        (assert_constraint_usage 'originPointConstraint'
          (documentation)
          (result_expr_membership))
        (item_usage composite 'subSpatialItems' : 'SpatialItems::SpatialItem'[item_def] :> 'subitems'[unresolved]
          (multiplicity_range [1..*])
          (item_usage reference :>> 'SpatialItem::localClock'[unresolved] :>> 'subitems::localClock'[unresolved]))
        (part_usage composite 'subSpatialParts' : 'SpatialItems::SpatialItem'[item_def] :> 'SpatialItems::SpatialItem::subSpatialItems'[item_usage] :> 'subparts'[unresolved]
          (multiplicity_range [1..*])
          (item_usage reference :>> 'SpatialItem::localClock'[unresolved] :>> 'subSpatialItems::localClock'[unresolved] :>> 'subparts::localClock'[unresolved]))
        (item_usage composite 'componentItems' : 'SpatialItems::SpatialItem'[item_def] :> 'SpatialItems::SpatialItem::subSpatialItems'[item_usage]
          (multiplicity_range [1..*])
          (documentation)
          (item_usage reference :>> 'SpatialItem::localClock'[unresolved] :>> 'subSpatialItems::localClock'[unresolved]
            (feature_value (default =)))
          (attribute_usage composite :>> 'SpatialItems::SpatialItem::coordinateFrame'[attribute_usage]
            (attribute_usage composite :>> 'mRefs'[unresolved]
              (feature_value (default =)))
            (attribute_usage composite :>> 'transformation'[unresolved]
              (multiplicity_range [1])
              (feature_value (default =))
              (attribute_usage composite :>> 'source'[unresolved]
                (feature_value (default =))))))
        (attribute_usage composite 'cunionNum' : 'Natural'[unresolved]
          (multiplicity_range [1])
          (feature_value (=)))
        (attribute_usage composite 'componentUnion' :> 'unionsOf'[unresolved]
          (multiplicity_range [?])
          (documentation)
          (item_usage composite :>> 'elements'[unresolved] : 'SpatialItems::SpatialItem'[item_def]
            (multiplicity_range [1..*])
            (feature_value (=))))
        (part_usage composite 'componentParts' : 'SpatialItems::SpatialItem'[item_def] :> 'SpatialItems::SpatialItem::componentItems'[item_usage] :> 'SpatialItems::SpatialItem::subSpatialParts'[part_usage]
          (multiplicity_range [1..*])
          (item_usage reference :>> 'SpatialItem::localClock'[unresolved] :>> 'componentItems::localClock'[unresolved] :>> 'subSpatialParts::localClock'[unresolved] :>> 'subparts::localClock'[unresolved])))
      (calculation_def 'PositionOf' :> 'SpatialFrames::PositionOf'[unresolved]
        (documentation)
        (reference_usage in reference 'point' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'timeInstant' : 'TimeInstantValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'enclosingItem' :>> 'frame'[unresolved] : 'SpatialItems::SpatialItem'[item_def]
          (multiplicity_range [1]))
        (reference_usage in reference 'clock' : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'positionVector' : 'Position3dVector'[unresolved]
            (multiplicity_range [1])
            (attribute_usage composite :>> 'mRef'[unresolved]
              (feature_value (=))))))
      (calculation_def 'CurrentPositionOf' :> 'SpatialFrames::CurrentPositionOf'[unresolved]
        (documentation)
        (reference_usage in reference 'point' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'enclosingItem' :>> 'frame'[unresolved] : 'SpatialItems::SpatialItem'[item_def]
          (multiplicity_range [1]))
        (reference_usage in reference 'clock' : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'positionVector' : 'Position3dVector'[unresolved]
            (multiplicity_range [1])
            (attribute_usage composite :>> 'mRef'[unresolved]
              (feature_value (=))))))
      (calculation_def 'DisplacementOf' :> 'SpatialFrames::DisplacementOf'[unresolved]
        (documentation)
        (reference_usage in reference 'point1' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'point2' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'timeInstant' : 'TimeInstantValue'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'spatialItem' :>> 'frame'[unresolved] : 'SpatialItems::SpatialItem'[item_def]
          (multiplicity_range [1]))
        (reference_usage in reference 'clock' : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'displacementVector' : 'Displacement3dVector'[unresolved]
            (multiplicity_range [1])
            (attribute_usage composite :>> 'mRef'[unresolved]
              (feature_value (=))))))
      (calculation_def 'CurrentDisplacementOf' :> 'SpatialFrames::CurrentDisplacementOf'[unresolved]
        (documentation)
        (reference_usage in reference 'point1' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'point2' : 'Point'[unresolved]
          (multiplicity_range [1]))
        (reference_usage in reference 'spatialItem' :>> 'frame'[unresolved] : 'SpatialItems::SpatialItem'[item_def]
          (multiplicity_range [1]))
        (reference_usage in reference 'clock' : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out 'displacementVector' : 'Displacement3dVector'[unresolved]
            (multiplicity_range [1])
            (attribute_usage composite :>> 'mRef'[unresolved]
              (feature_value (=)))))))))
~~~
