# META
~~~ini
description=SysML Example (Geometry): VehicleGeometryAndCoordinateFrames
type=file
~~~
# SOURCE
~~~sysml
package VehicleGeometryAndCoordinateFrames {
    private import TrigFunctions::*;
    private import ISQ::*;
    private import SI::*;
    private import Time::*;

    private import ShapeItems::*;
    private import SpatialItems::*;

    private import MeasurementReferences::CoordinateFrame;
    private import MeasurementReferences::TranslationRotationSequence;
    private import MeasurementReferences::Translation;
    private import MeasurementReferences::Rotation;
    
    private import Collections::Array;
    private import ScalarValues::Boolean;
    private import ScalarValues::Real;
    private import ScalarValues::Natural;
	private import ControlFunctions::forAll;
    
    part def Vehicle :> SpatialItem;

    part def Chassis :> SpatialItem {
        item :>> shape = new Box(4800 [mm], 1840 [mm], 1350 [mm]);
    }

    part def Wheel :> SpatialItem {
		doc
		/*
		 * Generic wheel with lugbolts
		 * 
		 * The radius is estimated for a 22 inch hub plus 110 mm tire height.
		 * The wheel width is equal to the cylinder height.
		 * The wheel has 5 lugbolts that are evenly distributed along a circle centered at the wheel's center.
		 */
	
        item :>> shape : Cylinder {
            :>> radius = 22/2*25.4 + 110 [mm]; 
            :>> height = 220 [mm];
        }
        attribute <wcf> wheelCoordinateFrame : CoordinateFrame;
        
        attribute numberOfBolts : Natural = 5;	
		part lugBolts : LugBolt[1..numberOfBolts] :> subSpatialParts;
		
		/* 
		 * As an example of a more involved placement of composite parts, constrain the positions of the coordinate frame origins 
		 * of the lugbolts to a circle with radius lbpr distributed evenly over 360°.
		 */
        attribute <lbpr> lugBoltPlacementRadius :>> radius default 60 [mm];
		private attribute lugBoltDistributionAngle :>> planeAngle = 360/numberOfBolts ['°'];
        private attribute lbda : Real = lugBoltDistributionAngle.num * (pi/180); // lugBoltDistributionAngle in radian
		assert constraint {
			(1..numberOfBolts)->forAll {
				in i : Natural;
				private attribute lbcf = lugBolts#(i).coordinateFrame; 
				private attribute trs : TranslationRotationSequence {
					:>> source = wcf;
					:>> target = lbcf;
					:>> elements = new Translation((lbpr*cos((i-1)*lbda), lbpr*sin((i-1)*lbda), -8)[wcf]); 
				}
				lbcf.transformation == trs
			}
		}
    }

    part def LugBolt :> SpatialItem {
    	item :>> shape : Cylinder {
    		:>> radius = 14 [mm];
    		:>> height = 40 [mm];
    	}
    }

    part vehicle : Vehicle, SpatialItem {
		/* 
		 * Vehicle frame origin at center of bottom plate of chassis
		 * with +Z upwards and +X in the forward (front) direction
		 */
        attribute datum :>> coordinateFrame {
            :>> mRefs = (mm, mm, mm);
        }

        part chassis : Chassis[1] :> componentParts {
			attribute :>> coordinateFrame {
				attribute :>> transformation : TranslationRotationSequence {
	 	          	attribute :>> source = datum;
                	attribute :>> elements = new Translation((-(shape as Box).length/2, -(shape as Box).width/2, 0)[datum]);
                }
			}
        }

        private attribute plusXAxis : Array { :>> dimensions = 3; :>> elements : Real[3] = (1, 0, 0); }
        private attribute frontWheelXShift : Real = 1670;
        private attribute rearWheelXShift : Real = -1820;
        private attribute wheelYShift : Real = 720;

        part leftFrontWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
	            	attribute :>> source = datum;
                	attribute :>> elements = (new Translation((frontWheelXShift, wheelYShift, 80)[datum]), new Rotation(plusXAxis[datum], -90['°']));
                }
            }
        }
        part rightFrontWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                	attribute :>> source = datum;
                	attribute :>> elements = (new Translation((frontWheelXShift, -wheelYShift, 80)[datum]), new Rotation((1, 0, 0)[datum], 90['°']));
                }
            }
        }
        part leftRearWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                	attribute :>> source = datum;
                	attribute :>> elements = (new Translation((rearWheelXShift, wheelYShift, 80)[datum]), new Rotation((1, 0, 0)[datum], 90['°']));
                }
            }
        }
        part rightRearWheel : Wheel[1] :> componentParts {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
					attribute :>> source = datum;
                	attribute :>> elements = (new Translation((rearWheelXShift, -wheelYShift, 80)[datum]), new Rotation((-1, 0, 0)[datum], 90['°']));
                }
            }
        }
    }
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
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
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Eq,Ident,Ident,OpenParen,DecimalValue,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,Ident,CloseSquare,CloseParen,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Slash,DecimalValue,Star,DecimalValue,Dot,DecimalValue,Plus,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Ident,CloseSquare,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,ColonGtGt,Ident,KwDefault,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPrivate,KwAttribute,Ident,ColonGtGt,Ident,Eq,DecimalValue,Slash,Ident,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Star,OpenParen,Ident,Slash,DecimalValue,CloseParen,Semicolon,LineComment,
KwAssert,KwConstraint,OpenCurly,
OpenParen,DecimalValue,DotDot,Ident,CloseParen,Arrow,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwPrivate,KwAttribute,Ident,Eq,Ident,Hash,OpenParen,Ident,CloseParen,Dot,Ident,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Ident,OpenParen,OpenParen,Ident,Star,Ident,OpenParen,OpenParen,Ident,Minus,DecimalValue,CloseParen,Star,Ident,CloseParen,Comma,Ident,Star,Ident,OpenParen,OpenParen,Ident,Minus,DecimalValue,CloseParen,Star,Ident,CloseParen,Comma,Minus,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Semicolon,
CloseCurly,
Ident,Dot,Ident,EqEq,Ident,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,Comma,Ident,OpenCurly,
RegularComment,
KwAttribute,Ident,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Ident,OpenParen,OpenParen,Minus,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Slash,DecimalValue,Comma,Minus,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Slash,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,Minus,DecimalValue,Semicolon,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Comma,Ident,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,Ident,Ident,OpenParen,Ident,OpenSquare,Ident,CloseSquare,Comma,Minus,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Comma,Minus,Ident,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Comma,Ident,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Comma,Minus,Ident,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,Comma,Ident,Ident,OpenParen,OpenParen,Minus,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleGeometryAndCoordinateFrames'
    (import_decl private 'TrigFunctions::*')
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private 'Time::*')
    (import_decl private 'ShapeItems::*')
    (import_decl private 'SpatialItems::*')
    (import_decl private 'MeasurementReferences::CoordinateFrame')
    (import_decl private 'MeasurementReferences::TranslationRotationSequence')
    (import_decl private 'MeasurementReferences::Translation')
    (import_decl private 'MeasurementReferences::Rotation')
    (import_decl private 'Collections::Array')
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'ControlFunctions::forAll')
    (part_def 'Vehicle' :> 'SpatialItem')
    (part_def 'Chassis' :> 'SpatialItem'
      (item_usage :>> 'shape' value))
    (part_def 'Wheel' :> 'SpatialItem'
      (documentation)
      (item_usage :>> 'shape' : 'Cylinder'
        (default_ref_usage :>> 'radius' value)
        (default_ref_usage :>> 'height' value))
      (attribute_usage 'wheelCoordinateFrame' : 'CoordinateFrame')
      (attribute_usage 'numberOfBolts' : 'Natural' value)
      (part_usage 'lugBolts' : 'LugBolt' :> 'subSpatialParts' multiplicity)
      (comment)
      (attribute_usage 'lugBoltPlacementRadius' :>> 'radius' value)
      (attribute_usage private 'lugBoltDistributionAngle' :>> 'planeAngle' value)
      (attribute_usage private 'lbda' : 'Real' value)
      (line_comment)
      (sysml_decl
        (result_expr_member)))
    (part_def 'LugBolt' :> 'SpatialItem'
      (item_usage :>> 'shape' : 'Cylinder'
        (default_ref_usage :>> 'radius' value)
        (default_ref_usage :>> 'height' value)))
    (part_usage 'vehicle' : 'Vehicle', 'SpatialItem'
      (comment)
      (attribute_usage 'datum' :>> 'coordinateFrame'
        (default_ref_usage :>> 'mRefs' value))
      (part_usage 'chassis' : 'Chassis' :> 'componentParts' multiplicity
        (attribute_usage :>> 'coordinateFrame'
          (attribute_usage :>> 'transformation' : 'TranslationRotationSequence'
            (attribute_usage :>> 'source' value)
            (attribute_usage :>> 'elements' value))))
      (attribute_usage private 'plusXAxis' : 'Array'
        (default_ref_usage :>> 'dimensions' value)
        (default_ref_usage :>> 'elements' : 'Real' multiplicity value))
      (attribute_usage private 'frontWheelXShift' : 'Real' value)
      (attribute_usage private 'rearWheelXShift' : 'Real' value)
      (attribute_usage private 'wheelYShift' : 'Real' value)
      (part_usage 'leftFrontWheel' : 'Wheel' :> 'componentParts' multiplicity
        (attribute_usage :>> 'coordinateFrame'
          (attribute_usage :>> 'transformation' : 'TranslationRotationSequence'
            (attribute_usage :>> 'source' value)
            (attribute_usage :>> 'elements' value))))
      (part_usage 'rightFrontWheel' : 'Wheel' :> 'componentParts' multiplicity
        (attribute_usage :>> 'coordinateFrame'
          (attribute_usage :>> 'transformation' : 'TranslationRotationSequence'
            (attribute_usage :>> 'source' value)
            (attribute_usage :>> 'elements' value))))
      (part_usage 'leftRearWheel' : 'Wheel' :> 'componentParts' multiplicity
        (attribute_usage :>> 'coordinateFrame'
          (attribute_usage :>> 'transformation' : 'TranslationRotationSequence'
            (attribute_usage :>> 'source' value)
            (attribute_usage :>> 'elements' value))))
      (part_usage 'rightRearWheel' : 'Wheel' :> 'componentParts' multiplicity
        (attribute_usage :>> 'coordinateFrame'
          (attribute_usage :>> 'transformation' : 'TranslationRotationSequence'
            (attribute_usage :>> 'source' value)
            (attribute_usage :>> 'elements' value)))))))
~~~
# FORMAT
~~~sysml
package VehicleGeometryAndCoordinateFrames {
    private import TrigFunctions::*;
    private import ISQ::*;
    private import SI::*;
    private import Time::*;

    private import ShapeItems::*;
    private import SpatialItems::*;

    private import MeasurementReferences::CoordinateFrame;
    private import MeasurementReferences::TranslationRotationSequence;
    private import MeasurementReferences::Translation;
    private import MeasurementReferences::Rotation;

    private import Collections::Array;
    private import ScalarValues::Boolean;
    private import ScalarValues::Real;
    private import ScalarValues::Natural;
    private import ControlFunctions::forAll;

    part def Vehicle :> SpatialItem;

    part def Chassis :> SpatialItem {
        item :>> shape = new Box(4800 [mm], 1840 [mm], 1350 [mm]);
    }

    part def Wheel :> SpatialItem {
        doc /*
		 * Generic wheel with lugbolts
		 * 
		 * The radius is estimated for a 22 inch hub plus 110 mm tire height.
		 * The wheel width is equal to the cylinder height.
		 * The wheel has 5 lugbolts that are evenly distributed along a circle centered at the wheel's center.
		 */

        item :>> shape : Cylinder {
            :>> radius = 22/2*25.4 + 110 [mm];
            :>> height = 220 [mm];
        }
        attribute <wcf> wheelCoordinateFrame : CoordinateFrame;

        attribute numberOfBolts : Natural = 5;
        part lugBolts : LugBolt :> subSpatialParts [1..numberOfBolts];

        /* 
		 * As an example of a more involved placement of composite parts, constrain the positions of the coordinate frame origins 
		 * of the lugbolts to a circle with radius lbpr distributed evenly over 360°.
		 */
        attribute <lbpr> lugBoltPlacementRadius :>> radius default = 60 [mm];
        private attribute lugBoltDistributionAngle :>> planeAngle = 360/numberOfBolts ['°'];
        private attribute lbda : Real = lugBoltDistributionAngle.num * (pi/180);
        // lugBoltDistributionAngle in radian
        assert constraint {
            = (1 .. numberOfBolts)->forAll {
				in i : Natural;
				private attribute lbcf = lugBolts#(i).coordinateFrame; 
				private attribute trs : TranslationRotationSequence {
					:>> source = wcf;
					:>> target = lbcf;
					:>> elements = new Translation((lbpr*cos((i-1)*lbda), lbpr*sin((i-1)*lbda), -8)[wcf]); 
				}
				lbcf.transformation == trs
			};
        }
    }

    part def LugBolt :> SpatialItem {
        item :>> shape : Cylinder {
            :>> radius = 14 [mm];
            :>> height = 40 [mm];
        }
    }

    part vehicle : Vehicle, SpatialItem {
        /* 
		 * Vehicle frame origin at center of bottom plate of chassis
		 * with +Z upwards and +X in the forward (front) direction
		 */
        attribute datum :>> coordinateFrame {
            :>> mRefs = (mm, mm, mm);
        }

        part chassis : Chassis :> componentParts [1] {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                    attribute :>> source = datum;
                    attribute :>> elements = new Translation((-(shape as Box).length/2, -(shape as Box).width/2, 0)[datum]);
                }
            }
        }

        private attribute plusXAxis : Array {
            :>> dimensions = 3;
            :>> elements : Real [3] = (1, 0, 0);
        }
        private attribute frontWheelXShift : Real = 1670;
        private attribute rearWheelXShift : Real = -1820;
        private attribute wheelYShift : Real = 720;

        part leftFrontWheel : Wheel :> componentParts [1] {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                    attribute :>> source = datum;
                    attribute :>> elements = (new Translation((frontWheelXShift, wheelYShift, 80)[datum]), new Rotation(plusXAxis[datum], -90['°']));
                }
            }
        }
        part rightFrontWheel : Wheel :> componentParts [1] {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                    attribute :>> source = datum;
                    attribute :>> elements = (new Translation((frontWheelXShift, -wheelYShift, 80)[datum]), new Rotation((1, 0, 0)[datum], 90['°']));
                }
            }
        }
        part leftRearWheel : Wheel :> componentParts [1] {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                    attribute :>> source = datum;
                    attribute :>> elements = (new Translation((rearWheelXShift, wheelYShift, 80)[datum]), new Rotation((1, 0, 0)[datum], 90['°']));
                }
            }
        }
        part rightRearWheel : Wheel :> componentParts [1] {
            attribute :>> coordinateFrame {
                attribute :>> transformation : TranslationRotationSequence {
                    attribute :>> source = datum;
                    attribute :>> elements = (new Translation((rearWheelXShift, -wheelYShift, 80)[datum]), new Rotation((-1, 0, 0)[datum], 90['°']));
                }
            }
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'planeAngle'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'CoordinateFrame'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'planeAngle'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
~~~
# SMG
~~~
(model
  (namespace
    (package 'VehicleGeometryAndCoordinateFrames'
      (namespace_import private -> 'TrigFunctions'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'Time'[unresolved])
      (namespace_import private -> 'ShapeItems'[unresolved])
      (namespace_import private -> 'SpatialItems'[unresolved])
      (membership_import private -> 'MeasurementReferences::CoordinateFrame'[unresolved])
      (membership_import private -> 'MeasurementReferences::TranslationRotationSequence'[unresolved])
      (membership_import private -> 'MeasurementReferences::Translation'[unresolved])
      (membership_import private -> 'MeasurementReferences::Rotation'[unresolved])
      (membership_import private -> 'Collections::Array'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (part_def 'Vehicle' :> 'SpatialItem'[unresolved])
      (part_def 'Chassis' :> 'SpatialItem'[unresolved]
        (item_usage composite :>> 'shape'[unresolved]
          (feature_value (=))))
      (part_def 'Wheel' :> 'SpatialItem'[unresolved]
        (documentation)
        (item_usage composite :>> 'shape'[unresolved] : 'Cylinder'[unresolved]
          (reference_usage reference :>> 'radius'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'height'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'wheelCoordinateFrame' : 'CoordinateFrame'[unresolved])
        (attribute_usage composite 'numberOfBolts' : 'Natural'[unresolved]
          (feature_value (=)))
        (part_usage composite 'lugBolts' : 'VehicleGeometryAndCoordinateFrames::LugBolt'[part_def] :> 'subSpatialParts'[unresolved]
          (multiplicity_range [1..?]))
        (attribute_usage composite 'lugBoltPlacementRadius' :>> 'radius'[unresolved]
          (feature_value (default =)))
        (attribute_usage composite 'lugBoltDistributionAngle' :>> 'planeAngle'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'lbda' : 'Real'[unresolved]
          (feature_value (=)))
        (assert_constraint_usage
          (result_expr_membership)))
      (part_def 'LugBolt' :> 'SpatialItem'[unresolved]
        (item_usage composite :>> 'shape'[unresolved] : 'Cylinder'[unresolved]
          (reference_usage reference :>> 'radius'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'height'[unresolved]
            (feature_value (=)))))
      (part_usage 'vehicle' : 'VehicleGeometryAndCoordinateFrames::Vehicle'[part_def] : 'SpatialItem'[unresolved]
        (attribute_usage composite 'datum' :>> 'coordinateFrame'[unresolved]
          (reference_usage reference :>> 'mRefs'[unresolved]
            (feature_value (=))))
        (part_usage composite 'chassis' : 'VehicleGeometryAndCoordinateFrames::Chassis'[part_def] :> 'componentParts'[unresolved]
          (multiplicity_range [1])
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (attribute_usage composite :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (attribute_usage composite :>> 'source'[unresolved]
                (feature_value (=)))
              (attribute_usage composite :>> 'elements'[unresolved]
                (feature_value (=))))))
        (attribute_usage composite 'plusXAxis' : 'Array'[unresolved]
          (reference_usage reference :>> 'dimensions'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'elements'[unresolved] : 'Real'[unresolved]
            (multiplicity_range [3])
            (feature_value (=))))
        (attribute_usage composite 'frontWheelXShift' : 'Real'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'rearWheelXShift' : 'Real'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'wheelYShift' : 'Real'[unresolved]
          (feature_value (=)))
        (part_usage composite 'leftFrontWheel' : 'VehicleGeometryAndCoordinateFrames::Wheel'[part_def] :> 'componentParts'[unresolved]
          (multiplicity_range [1])
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (attribute_usage composite :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (attribute_usage composite :>> 'source'[unresolved]
                (feature_value (=)))
              (attribute_usage composite :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'rightFrontWheel' : 'VehicleGeometryAndCoordinateFrames::Wheel'[part_def] :> 'componentParts'[unresolved]
          (multiplicity_range [1])
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (attribute_usage composite :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (attribute_usage composite :>> 'source'[unresolved]
                (feature_value (=)))
              (attribute_usage composite :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'leftRearWheel' : 'VehicleGeometryAndCoordinateFrames::Wheel'[part_def] :> 'componentParts'[unresolved]
          (multiplicity_range [1])
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (attribute_usage composite :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (attribute_usage composite :>> 'source'[unresolved]
                (feature_value (=)))
              (attribute_usage composite :>> 'elements'[unresolved]
                (feature_value (=))))))
        (part_usage composite 'rightRearWheel' : 'VehicleGeometryAndCoordinateFrames::Wheel'[part_def] :> 'componentParts'[unresolved]
          (multiplicity_range [1])
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (attribute_usage composite :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (attribute_usage composite :>> 'source'[unresolved]
                (feature_value (=)))
              (attribute_usage composite :>> 'elements'[unresolved]
                (feature_value (=))))))))))
~~~
