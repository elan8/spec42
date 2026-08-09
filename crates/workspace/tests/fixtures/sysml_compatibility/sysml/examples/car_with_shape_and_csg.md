# META
~~~ini
description=SysML Example (Geometry): CarWithShapeAndCSG
type=file
~~~
# SOURCE
~~~sysml
package CarWithShapeAndCSG {
	private import SpatialItems::*;
	private import ShapeItems::*;
	private import Objects::Point;
	private import Quantities::VectorQuantityValue;
	private import MeasurementReferences::CoordinateFrame;
	private import MeasurementReferences::TranslationRotationSequence;
	private import MeasurementReferences::Translation;
	private import MeasurementReferences::Rotation;
	private import SI::*;

	part def Car :> SpatialItem {
		doc
		/*
		 * Car with simple engine
		 */
	
        item :>> shape = new Cuboid(4800 [mm], 1840 [mm], 1350 [mm]);

        attribute datum :>> coordinateFrame {
            :>> mRefs = (mm, mm, mm);
        }

		part powerSource : Engine [1] :> componentParts {
			:>> ecf { 
				:>> mRefs = datum.mRefs;
				:>> transformation : TranslationRotationSequence {
					:>> source = datum;
					:>> elements = ( new Translation((3800, (1840-190)/2, 40)[datum]) );
				}
			}
		}
	}

	part def Engine :> SpatialItem {
		doc
		/*
		 * Simple 2-cylinder engine
		 * 
		 * Note: The engine shape is modeled as a rectangular box with two cylindrical holes, a gross simplification.
		 */
	
		item :>> shape [1];
		
		attribute <ecf> engineCoordinateFrame :>> coordinateFrame;		

		part rawEngineBlock :> subSpatialParts [1] {
			item :>> shape : Box [1] {
	    		:>> length = 300 [mm];
	    		:>> width = 190 [mm];
	    		:>> height = 330 [mm];
			}
		}
		
		private attribute rearCylinderSpacing = 90 [mm];
		private item cylinder1  :> subSpatialParts [1] {
			item :>> shape : Cylinder [1] {
	    		:>> radius = 55 [mm];
	    		:>> height = 350 [mm];
			}
			attribute :>> coordinateFrame {
				:>> transformation : TranslationRotationSequence {
					:>> source = ecf;
					:>> elements = (new Translation( (rearCylinderSpacing, rawEngineBlock.shape.width/2, -10)[ecf]));
				}
			}
		}
		
		private attribute cylinderSpacing = 2*cylinder1.shape.radius + 20 [mm];
		private item cylinder2  :> subSpatialParts [1] {
			item :>> shape : Cylinder [1] {
	    		:>> radius = cylinder1.shape.radius;
	    		:>> height = cylinder1.shape.height;
			}
			attribute :>> coordinateFrame {
				:>> transformation : TranslationRotationSequence {
					:>> source = ecf;
					:>> elements = ( new Translation((rearCylinderSpacing + cylinderSpacing, rawEngineBlock.shape.width/2, -10)[ecf]) );
				}
			}
		}

		/* CSG difference of rawEngineBlock minus cylinder1 minus cylinder2 */
		attribute :> differencesOf[1] {
			item :>> elements = (rawEngineBlock, cylinder1, cylinder2);
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,Eq,Ident,Ident,OpenParen,DecimalValue,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,Ident,CloseSquare,Comma,DecimalValue,OpenSquare,Ident,CloseSquare,CloseParen,Semicolon,
KwAttribute,Ident,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,DecimalValue,Comma,OpenParen,DecimalValue,Minus,DecimalValue,CloseParen,Slash,DecimalValue,Comma,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,ColonGtGt,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPrivate,KwAttribute,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPrivate,KwItem,Ident,ColonGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Comma,Ident,Dot,Ident,Dot,Ident,Slash,DecimalValue,Comma,Minus,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPrivate,KwAttribute,Ident,Eq,DecimalValue,Star,Ident,Dot,Ident,Dot,Ident,Plus,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPrivate,KwItem,Ident,ColonGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,OpenParen,Ident,Ident,OpenParen,OpenParen,Ident,Plus,Ident,Comma,Ident,Dot,Ident,Dot,Ident,Slash,DecimalValue,Comma,Minus,DecimalValue,CloseParen,OpenSquare,Ident,CloseSquare,CloseParen,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,ColonGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwItem,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'CarWithShapeAndCSG'
    (import_decl private 'SpatialItems::*')
    (import_decl private 'ShapeItems::*')
    (import_decl private 'Objects::Point')
    (import_decl private 'Quantities::VectorQuantityValue')
    (import_decl private 'MeasurementReferences::CoordinateFrame')
    (import_decl private 'MeasurementReferences::TranslationRotationSequence')
    (import_decl private 'MeasurementReferences::Translation')
    (import_decl private 'MeasurementReferences::Rotation')
    (import_decl private 'SI::*')
    (part_def 'Car' :> 'SpatialItem'
      (documentation)
      (item_usage :>> 'shape' value)
      (attribute_usage 'datum' :>> 'coordinateFrame'
        (default_ref_usage :>> 'mRefs' value))
      (part_usage 'powerSource' : 'Engine' :> 'componentParts' multiplicity
        (default_ref_usage :>> 'ecf'
          (default_ref_usage :>> 'mRefs' value)
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'source' value)
            (default_ref_usage :>> 'elements' value)))))
    (part_def 'Engine' :> 'SpatialItem'
      (documentation)
      (item_usage :>> 'shape' multiplicity)
      (attribute_usage 'engineCoordinateFrame' :>> 'coordinateFrame')
      (part_usage 'rawEngineBlock' :> 'subSpatialParts' multiplicity
        (item_usage :>> 'shape' : 'Box' multiplicity
          (default_ref_usage :>> 'length' value)
          (default_ref_usage :>> 'width' value)
          (default_ref_usage :>> 'height' value)))
      (attribute_usage private 'rearCylinderSpacing' value)
      (item_usage private 'cylinder1' :> 'subSpatialParts' multiplicity
        (item_usage :>> 'shape' : 'Cylinder' multiplicity
          (default_ref_usage :>> 'radius' value)
          (default_ref_usage :>> 'height' value))
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'source' value)
            (default_ref_usage :>> 'elements' value))))
      (attribute_usage private 'cylinderSpacing' value)
      (item_usage private 'cylinder2' :> 'subSpatialParts' multiplicity
        (item_usage :>> 'shape' : 'Cylinder' multiplicity
          (default_ref_usage :>> 'radius' value)
          (default_ref_usage :>> 'height' value))
        (attribute_usage :>> 'coordinateFrame'
          (default_ref_usage :>> 'transformation' : 'TranslationRotationSequence'
            (default_ref_usage :>> 'source' value)
            (default_ref_usage :>> 'elements' value))))
      (comment)
      (attribute_usage :> 'differencesOf' multiplicity
        (item_usage :>> 'elements' value)))))
~~~
# FORMAT
~~~sysml
package CarWithShapeAndCSG {
	private import SpatialItems::*;
	private import ShapeItems::*;
	private import Objects::Point;
	private import Quantities::VectorQuantityValue;
	private import MeasurementReferences::CoordinateFrame;
	private import MeasurementReferences::TranslationRotationSequence;
	private import MeasurementReferences::Translation;
	private import MeasurementReferences::Rotation;
	private import SI::*;

	part def Car :> SpatialItem {
		doc
		/*
		 * Car with simple engine
		 */
	
        item :>> shape = new Cuboid(4800 [mm], 1840 [mm], 1350 [mm]);

        attribute datum :>> coordinateFrame {
            :>> mRefs = (mm, mm, mm);
        }

		part powerSource : Engine [1] :> componentParts {
			:>> ecf { 
				:>> mRefs = datum.mRefs;
				:>> transformation : TranslationRotationSequence {
					:>> source = datum;
					:>> elements = ( new Translation((3800, (1840-190)/2, 40)[datum]) );
				}
			}
		}
	}

	part def Engine :> SpatialItem {
		doc
		/*
		 * Simple 2-cylinder engine
		 * 
		 * Note: The engine shape is modeled as a rectangular box with two cylindrical holes, a gross simplification.
		 */
	
		item :>> shape [1];
		
		attribute <ecf> engineCoordinateFrame :>> coordinateFrame;		

		part rawEngineBlock :> subSpatialParts [1] {
			item :>> shape : Box [1] {
	    		:>> length = 300 [mm];
	    		:>> width = 190 [mm];
	    		:>> height = 330 [mm];
			}
		}
		
		private attribute rearCylinderSpacing = 90 [mm];
		private item cylinder1  :> subSpatialParts [1] {
			item :>> shape : Cylinder [1] {
	    		:>> radius = 55 [mm];
	    		:>> height = 350 [mm];
			}
			attribute :>> coordinateFrame {
				:>> transformation : TranslationRotationSequence {
					:>> source = ecf;
					:>> elements = (new Translation( (rearCylinderSpacing, rawEngineBlock.shape.width/2, -10)[ecf]));
				}
			}
		}
		
		private attribute cylinderSpacing = 2*cylinder1.shape.radius + 20 [mm];
		private item cylinder2  :> subSpatialParts [1] {
			item :>> shape : Cylinder [1] {
	    		:>> radius = cylinder1.shape.radius;
	    		:>> height = cylinder1.shape.height;
			}
			attribute :>> coordinateFrame {
				:>> transformation : TranslationRotationSequence {
					:>> source = ecf;
					:>> elements = ( new Translation((rearCylinderSpacing + cylinderSpacing, rawEngineBlock.shape.width/2, -10)[ecf]) );
				}
			}
		}

		/* CSG difference of rawEngineBlock minus cylinder1 minus cylinder2 */
		attribute :> differencesOf[1] {
			item :>> elements = (rawEngineBlock, cylinder1, cylinder2);
		}
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'differencesOf'
semantic.unresolved_name 'elements'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'componentParts'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'SpatialItem'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'subSpatialParts'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Cylinder'
semantic.unresolved_name 'radius'
semantic.unresolved_name 'height'
semantic.unresolved_name 'coordinateFrame'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'TranslationRotationSequence'
semantic.unresolved_name 'source'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'differencesOf'
semantic.unresolved_name 'elements'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (name "CarWithShapeAndCSG") (declared-name "CarWithShapeAndCSG")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (name "Car") (declared-name "Car") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::datum"))) (name "datum") (declared-name "datum") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (name "powerSource") (declared-name "powerSource") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (name "ecf") (declared-name "ecf") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine")))))
              )
            )
            (element (kind "item") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (name "shape") (declared (properties (composite true) (reference false)) (feature-value (kind bound) (expression (kind "constructor") (reference "Cuboid") (arguments (argument (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 4800)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm"))))))) (argument (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1840)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm"))))))) (argument (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1350)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm"))))))))))) (effective (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (role feature-value))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::CoordinateFrame"))) (name "CoordinateFrame") (declared-name "CoordinateFrame"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (name "Engine") (declared-name "Engine") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (name "cylinder1") (declared-name "cylinder1") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine")))))
              )
            )
            (element (kind "item") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (name "cylinder2") (declared-name "cylinder2") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (name "coordinateFrame") (declared-name "coordinateFrame") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (name "cylinderSpacing") (declared-name "cylinderSpacing") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "+") (children (expression (kind "binary") (operator "*") (children (expression (kind "integerLiteral") (literal 2)) (expression (kind "memberAccess") (reference "radius") (children (expression (kind "memberAccess") (reference "shape") (children (expression (kind "featureReference") (reference "cylinder1")))))))) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 20)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))) (name "engineCoordinateFrame") (declared-name "engineCoordinateFrame") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (name "rawEngineBlock") (declared-name "rawEngineBlock") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing"))) (name "rearCylinderSpacing") (declared-name "rearCylinderSpacing") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 90)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing"))) (role feature-value))))
            (element (kind "item") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (name "shape") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Point"))) (name "Point") (declared-name "Point"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Rotation"))) (name "Rotation") (declared-name "Rotation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Translation"))) (name "Translation") (declared-name "Translation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::TranslationRotationSequence"))) (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::_documentation"))) (to (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::_documentation"))) (to (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (to (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
