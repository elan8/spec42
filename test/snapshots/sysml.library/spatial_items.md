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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "30fa42f87fe9a8bbb62fddc8b434f34d95ca17c3a368f0d0f66c38e062e9d400") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SpatialItems"))) (kind "package") (name "SpatialItems") (declared-name "SpatialItems") (range (start (line 0) (character 0)) (end (line 0) (character 6129))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Clock"))) (kind "import") (name "Clock") (declared-name "Clock") (range (start (line 12) (character 1)) (end (line 12) (character 28))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::Clock") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 27))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (kind "calc def") (name "CurrentDisplacementOf") (declared-name "CurrentDisplacementOf") (range (start (line 151) (character 1)) (end (line 151) (character 537))) (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::_documentation"))) (kind "documentation") (name "") (range (start (line 151) (character 1)) (end (line 151) (character 537))) (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (range (start (line 161) (character 2)) (end (line 161) (character 53))) (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (authored (relationships (typing (reference "clock : Clock[1] default spatialItem.localClock") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point1"))) (kind "in out parameter") (name "point1") (declared-name "point1") (range (start (line 158) (character 2)) (end (line 158) (character 23))) (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (authored (relationships (typing (reference "point1 : Point[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point2"))) (kind "in out parameter") (name "point2") (declared-name "point2") (range (start (line 159) (character 2)) (end (line 159) (character 23))) (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (authored (relationships (typing (reference "point2 : Point[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind "in out parameter") (name "spatialItem") (declared-name "spatialItem") (range (start (line 160) (character 2)) (end (line 160) (character 46))) (parent (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf"))) (authored (relationships (typing (reference "spatialItem :>> 'frame' : SpatialItem[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (kind "calc def") (name "CurrentPositionOf") (declared-name "CurrentPositionOf") (range (start (line 118) (character 1)) (end (line 118) (character 489))) (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::_documentation"))) (kind "documentation") (name "") (range (start (line 118) (character 1)) (end (line 118) (character 489))) (parent (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (range (start (line 127) (character 2)) (end (line 127) (character 55))) (parent (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (authored (relationships (typing (reference "clock : Clock[1] default enclosingItem.localClock") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind "in out parameter") (name "enclosingItem") (declared-name "enclosingItem") (range (start (line 126) (character 2)) (end (line 126) (character 48))) (parent (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (authored (relationships (typing (reference "enclosingItem :>> 'frame' : SpatialItem[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::point"))) (kind "in out parameter") (name "point") (declared-name "point") (range (start (line 125) (character 2)) (end (line 125) (character 22))) (parent (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf"))) (authored (relationships (typing (reference "point : Point[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Displacement3dVector"))) (kind "import") (name "Displacement3dVector") (declared-name "Displacement3dVector") (range (start (line 17) (character 1)) (end (line 17) (character 42))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::Displacement3dVector") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 41))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (kind "calc def") (name "DisplacementOf") (declared-name "DisplacementOf") (range (start (line 133) (character 1)) (end (line 133) (character 690))) (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::_documentation"))) (kind "documentation") (name "") (range (start (line 133) (character 1)) (end (line 133) (character 690))) (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (range (start (line 145) (character 2)) (end (line 145) (character 53))) (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "clock : Clock[1] default spatialItem.localClock") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point1"))) (kind "in out parameter") (name "point1") (declared-name "point1") (range (start (line 141) (character 2)) (end (line 141) (character 23))) (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "point1 : Point[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point2"))) (kind "in out parameter") (name "point2") (declared-name "point2") (range (start (line 142) (character 2)) (end (line 142) (character 23))) (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "point2 : Point[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind "in out parameter") (name "spatialItem") (declared-name "spatialItem") (range (start (line 144) (character 2)) (end (line 144) (character 46))) (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "spatialItem :>> 'frame' : SpatialItem[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::timeInstant"))) (kind "in out parameter") (name "timeInstant") (declared-name "timeInstant") (range (start (line 143) (character 2)) (end (line 143) (character 39))) (parent (node (document "d0") (qualified-name "SpatialItems::DisplacementOf"))) (authored (relationships (typing (reference "timeInstant : TimeInstantValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 14) (character 1)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Point"))) (kind "import") (name "Point") (declared-name "Point") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Point") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::Position3dVector"))) (kind "import") (name "Position3dVector") (declared-name "Position3dVector") (range (start (line 16) (character 1)) (end (line 16) (character 38))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::Position3dVector") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 37))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (kind "calc def") (name "PositionOf") (declared-name "PositionOf") (range (start (line 101) (character 1)) (end (line 101) (character 620))) (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::_documentation"))) (kind "documentation") (name "") (range (start (line 101) (character 1)) (end (line 101) (character 620))) (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::clock"))) (kind "in out parameter") (name "clock") (declared-name "clock") (range (start (line 112) (character 2)) (end (line 112) (character 55))) (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (authored (relationships (typing (reference "clock : Clock[1] default enclosingItem.localClock") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind "in out parameter") (name "enclosingItem") (declared-name "enclosingItem") (range (start (line 111) (character 2)) (end (line 111) (character 48))) (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (authored (relationships (typing (reference "enclosingItem :>> 'frame' : SpatialItem[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::point"))) (kind "in out parameter") (name "point") (declared-name "point") (range (start (line 109) (character 2)) (end (line 109) (character 22))) (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (authored (relationships (typing (reference "point : Point[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::PositionOf::timeInstant"))) (kind "in out parameter") (name "timeInstant") (declared-name "timeInstant") (range (start (line 110) (character 2)) (end (line 110) (character 39))) (parent (node (document "d0") (qualified-name "SpatialItems::PositionOf"))) (authored (relationships (typing (reference "timeInstant : TimeInstantValue[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialFrame"))) (kind "import") (name "SpatialFrame") (declared-name "SpatialFrame") (range (start (line 8) (character 1)) (end (line 8) (character 44))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SpatialFrames::SpatialFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 43))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (kind "item def") (name "SpatialItem") (declared-name "SpatialItem") (range (start (line 22) (character 1)) (end (line 22) (character 2909))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialFrame") (range (start (line 22) (character 25)) (end (line 22) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::"))) (kind "ref") (name "") (range (start (line 28) (character 2)) (end (line 28) (character 239))) (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature)) (relationships (typing (reference "Clock") (range (start (line 28) (character 27)) (end (line 28) (character 33)))) (redefinition (reference "localClock") (range (start (line 28) (character 15)) (end (line 28) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::_documentation"))) (kind "documentation") (name "") (range (start (line 22) (character 1)) (end (line 22) (character 2909))) (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind "part") (name "componentParts") (declared-name "componentParts") (range (start (line 96) (character 2)) (end (line 96) (character 205))) (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpatialItem") (range (start (line 96) (character 24)) (end (line 96) (character 35)))) (subsetting (reference "componentItems") (range (start (line 96) (character 45)) (end (line 96) (character 59)))) (subsetting (reference "subSpatialParts") (range (start (line 96) (character 61)) (end (line 96) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (kind "attribute") (name "componentUnion") (declared-name "componentUnion") (range (start (line 87) (character 2)) (end (line 87) (character 217))) (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature) (visibility "private")) (relationships (subsetting (reference "unionsOf") (range (start (line 87) (character 49)) (end (line 87) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion::_documentation"))) (kind "documentation") (name "") (range (start (line 87) (character 2)) (end (line 87) (character 217))) (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 36) (character 2)) (end (line 36) (character 439))) (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThreeDCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame::_documentation"))) (kind "documentation") (name "") (range (start (line 36) (character 2)) (end (line 36) (character 439))) (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind "attribute") (name "cunionNum") (declared-name "cunionNum") (range (start (line 86) (character 2)) (end (line 86) (character 83))) (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "Natural") (range none)))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind "part") (name "subSpatialParts") (declared-name "subSpatialParts") (range (start (line 65) (character 8)) (end (line 65) (character 190))) (parent (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpatialItem") (range (start (line 65) (character 31)) (end (line 65) (character 42)))) (subsetting (reference "subSpatialItems") (range (start (line 65) (character 52)) (end (line 65) (character 67)))) (subsetting (reference "subparts") (range (start (line 65) (character 69)) (end (line 65) (character 77)))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::ThreeDCoordinateFrame"))) (kind "import") (name "ThreeDCoordinateFrame") (declared-name "ThreeDCoordinateFrame") (range (start (line 10) (character 1)) (end (line 10) (character 61))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::ThreeDCoordinateFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 60))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::TimeInstantValue"))) (kind "import") (name "TimeInstantValue") (declared-name "TimeInstantValue") (range (start (line 13) (character 1)) (end (line 13) (character 39))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::TimeInstantValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 38))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 9) (character 1)) (end (line 9) (character 48))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 47))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 6129))) (parent (node (document "d0") (qualified-name "SpatialItems"))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 20) (character 1)) (end (line 20) (character 41))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 16)) (end (line 20) (character 40))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (range (start (line 19) (character 1)) (end (line 19) (character 43))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 19) (character 16)) (end (line 19) (character 42))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::isZeroVector"))) (kind "import") (name "isZeroVector") (declared-name "isZeroVector") (range (start (line 18) (character 1)) (end (line 18) (character 46))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorFunctions::isZeroVector") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 16)) (end (line 18) (character 45))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::nullTransformation"))) (kind "import") (name "nullTransformation") (declared-name "nullTransformation") (range (start (line 11) (character 1)) (end (line 11) (character 58))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::nullTransformation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 57))))))
    (element (id (node (document "d0") (qualified-name "SpatialItems::universalCartesianSpatial3dCoordinateFrame"))) (kind "import") (name "universalCartesianSpatial3dCoordinateFrame") (declared-name "universalCartesianSpatial3dCoordinateFrame") (range (start (line 15) (character 1)) (end (line 15) (character 64))) (parent (node (document "d0") (qualified-name "SpatialItems"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::universalCartesianSpatial3dCoordinateFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 63))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Clock"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::Clock") (range (start (line 12) (character 16)) (end (line 12) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default spatialItem.localClock") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point1"))) (kind featureTyping) (ordinal 0)) (authored-target "point1 : Point[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::point2"))) (kind featureTyping) (ordinal 0)) (authored-target "point2 : Point[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentDisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0)) (authored-target "spatialItem :>> 'frame' : SpatialItem[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default enclosingItem.localClock") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0)) (authored-target "enclosingItem :>> 'frame' : SpatialItem[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::CurrentPositionOf::point"))) (kind featureTyping) (ordinal 0)) (authored-target "point : Point[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Displacement3dVector"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::Displacement3dVector") (range (start (line 17) (character 16)) (end (line 17) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default spatialItem.localClock") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point1"))) (kind featureTyping) (ordinal 0)) (authored-target "point1 : Point[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::point2"))) (kind featureTyping) (ordinal 0)) (authored-target "point2 : Point[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::spatialItem"))) (kind featureTyping) (ordinal 0)) (authored-target "spatialItem :>> 'frame' : SpatialItem[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::DisplacementOf::timeInstant"))) (kind featureTyping) (ordinal 0)) (authored-target "timeInstant : TimeInstantValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 14) (character 16)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Point"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Point") (range (start (line 7) (character 16)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::Position3dVector"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::Position3dVector") (range (start (line 16) (character 16)) (end (line 16) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::PositionOf::clock"))) (kind featureTyping) (ordinal 0)) (authored-target "clock : Clock[1] default enclosingItem.localClock") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::PositionOf::enclosingItem"))) (kind featureTyping) (ordinal 0)) (authored-target "enclosingItem :>> 'frame' : SpatialItem[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::PositionOf::point"))) (kind featureTyping) (ordinal 0)) (authored-target "point : Point[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::PositionOf::timeInstant"))) (kind featureTyping) (ordinal 0)) (authored-target "timeInstant : TimeInstantValue[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "SpatialFrames::SpatialFrame") (range (start (line 8) (character 16)) (end (line 8) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem"))) (kind specialization) (ordinal 0)) (authored-target "SpatialFrame") (range (start (line 22) (character 25)) (end (line 22) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::SpatialFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::"))) (kind featureTyping) (ordinal 0)) (authored-target "Clock") (range (start (line 28) (character 27)) (end (line 28) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::Clock")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::"))) (kind redefinition) (ordinal 0)) (authored-target "localClock") (range (start (line 28) (character 15)) (end (line 28) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind featureTyping) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 96) (character 24)) (end (line 96) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 0)) (authored-target "componentItems") (range (start (line 96) (character 45)) (end (line 96) (character 59))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentParts"))) (kind subsetting) (ordinal 1)) (authored-target "subSpatialParts") (range (start (line 96) (character 61)) (end (line 96) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::componentUnion"))) (kind subsetting) (ordinal 0)) (authored-target "unionsOf") (range (start (line 87) (character 49)) (end (line 87) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::coordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "ThreeDCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::ThreeDCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::cunionNum"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind featureTyping) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 65) (character 31)) (end (line 65) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SpatialItems::SpatialItem")))))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialItems") (range (start (line 65) (character 52)) (end (line 65) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::SpatialItem::subSpatialParts"))) (kind subsetting) (ordinal 1)) (authored-target "subparts") (range (start (line 65) (character 69)) (end (line 65) (character 77))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::ThreeDCoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::ThreeDCoordinateFrame") (range (start (line 10) (character 16)) (end (line 10) (character 60))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::TimeInstantValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::TimeInstantValue") (range (start (line 13) (character 16)) (end (line 13) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (range (start (line 9) (character 16)) (end (line 9) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 20) (character 16)) (end (line 20) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (range (start (line 19) (character 16)) (end (line 19) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::isZeroVector"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorFunctions::isZeroVector") (range (start (line 18) (character 16)) (end (line 18) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::nullTransformation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::nullTransformation") (range (start (line 11) (character 16)) (end (line 11) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SpatialItems::universalCartesianSpatial3dCoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::universalCartesianSpatial3dCoordinateFrame") (range (start (line 15) (character 16)) (end (line 15) (character 63))) (outcome (status unresolved)))
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
