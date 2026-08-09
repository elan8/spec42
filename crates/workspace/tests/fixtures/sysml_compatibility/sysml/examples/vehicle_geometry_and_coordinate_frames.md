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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames"))) (name "VehicleGeometryAndCoordinateFrames") (declared-name "VehicleGeometryAndCoordinateFrames")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import4"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::*#import5"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Array"))) (name "Array") (declared-name "Array"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis"))) (name "Chassis") (declared-name "Chassis") (declared)
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape"))) (name "shape") (declared (feature-value (kind bound) (expression (kind "constructor") (reference "Box") (arguments (argument (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 4800)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm"))))))) (argument (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1840)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm"))))))) (argument (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1350)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm"))))))))))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis::shape"))) (role feature-value))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::CoordinateFrame"))) (name "CoordinateFrame") (declared-name "CoordinateFrame"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt"))) (name "LugBolt") (declared-name "LugBolt") (declared)
          (contains
            (element (kind "item") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape"))) (name "shape") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::height"))) (name "height") (declared-name "height") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt::shape::radius"))) (name "radius") (declared-name "radius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Rotation"))) (name "Rotation") (declared-name "Rotation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Translation"))) (name "Translation") (declared-name "Translation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::TranslationRotationSequence"))) (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lbda"))) (name "lbda") (declared-name "lbda") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "memberAccess") (reference "num") (children (expression (kind "featureReference") (reference "lugBoltDistributionAngle")))) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "pi")) (expression (kind "integerLiteral") (literal 180)))))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lbda"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltDistributionAngle"))) (name "lugBoltDistributionAngle") (declared-name "lugBoltDistributionAngle") (declared (properties (ordered false) (unique true)) (multiplicity (lower unevaluated) (upper unevaluated) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "integerLiteral") (literal 360)) (expression (kind "featureReference") (reference "numberOfBolts")))))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltDistributionAngle"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBoltPlacementRadius"))) (name "lugBoltPlacementRadius") (declared-name "lugBoltPlacementRadius") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 60)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBolts"))) (name "lugBolts") (declared-name "lugBolts") (declared (properties (ordered false)) (multiplicity (lower 1) (upper unevaluated) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::numberOfBolts"))) (name "numberOfBolts") (declared-name "numberOfBolts") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal 5)))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::numberOfBolts"))) (role feature-value))))
            (element (kind "item") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape"))) (name "shape") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::height"))) (name "height") (declared-name "height") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::shape::radius"))) (name "radius") (declared-name "radius") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::wheelCoordinateFrame"))) (name "wheelCoordinateFrame") (declared-name "wheelCoordinateFrame") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::forAll"))) (name "forAll") (declared-name "forAll"))
        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))) (name "chassis") (declared-name "chassis") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::datum"))) (name "datum") (declared-name "datum") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))) (name "frontWheelXShift") (declared-name "frontWheelXShift") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal 1670)))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::frontWheelXShift"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))) (name "leftFrontWheel") (declared-name "leftFrontWheel") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))) (name "leftRearWheel") (declared-name "leftRearWheel") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::plusXAxis"))) (name "plusXAxis") (declared-name "plusXAxis") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))) (name "rearWheelXShift") (declared-name "rearWheelXShift") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1820)))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rearWheelXShift"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))) (name "rightFrontWheel") (declared-name "rightFrontWheel") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))) (name "rightRearWheel") (declared-name "rightRearWheel") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))) (name "wheelYShift") (declared-name "wheelYShift") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal 720)))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::wheelYShift"))) (role feature-value))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::_documentation"))) (to (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel::lugBolts"))) (to (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::LugBolt"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle"))) (to (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::chassis"))) (to (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Chassis"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftFrontWheel"))) (to (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::leftRearWheel"))) (to (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightFrontWheel"))) (to (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::vehicle::rightRearWheel"))) (to (node (document "d0") (qualified-name "VehicleGeometryAndCoordinateFrames::Wheel"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicle_geometry_and_coordinate_frames.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 4) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 4) (end 2 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 4) (end 3 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 4) (end 4 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 4) (end 6 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 4) (end 7 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 4) (end 9 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 4) (end 10 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 4) (end 11 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 4) (end 12 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 4) (end 14 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 4) (end 15 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 4) (end 16 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 4) (end 17 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 1) (end 18 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 4) (end 20 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 4) (end 22 110))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 23 8) (end 23 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 4) (end 26 1526))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 36 8) (end 36 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 8) (end 36 128))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 37 12) (end 37 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 37 12) (end 37 46))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 38 12) (end 38 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 38 12) (end 38 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 8) (end 40 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 8) (end 42 46))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 49 8) (end 49 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 49 8) (end 49 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 50 2) (end 50 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 8) (end 51 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 66 4) (end 66 139))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 67 5) (end 67 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 5) (end 67 95))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 68 6) (end 68 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 68 6) (end 68 27))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 69 6) (end 69 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 69 6) (end 69 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 78 8) (end 78 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 83 3) (end 83 288))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 91 8) (end 91 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 92 8) (end 92 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 93 8) (end 93 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 94 8) (end 94 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 97 12) (end 97 344))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 105 12) (end 105 347))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 113 12) (end 113 345))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 121 12) (end 121 335))
      )
    )
  )
)
~~~
