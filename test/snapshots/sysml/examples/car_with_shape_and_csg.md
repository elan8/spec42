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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "car_with_shape_and_csg.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 11 17) (end 11 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 28) (end 19 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 21) (end 23 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 35) (end 23 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 34 20) (end 34 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 44) (end 44 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 25) (end 46 40))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 54 2) (end 54 50))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 68 2) (end 68 73))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 83 2) (end 83 102))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1d3e8936b4ebdeed61bdc5d2c1441bbcbb77ae2fac103cb96a76be34ff46e896") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (kind "package") (name "CarWithShapeAndCSG") (declared-name "CarWithShapeAndCSG") (range (start (line 0) (character 0)) (end (line 0) (character 2520))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "SpatialItems::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 30))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "ShapeItems::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 26))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 1)) (end (line 9) (character 22))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 18))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (kind "part def") (name "Car") (declared-name "Car") (range (start (line 11) (character 1)) (end (line 11) (character 510))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem") (range (start (line 11) (character 17)) (end (line 11) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::_documentation"))) (kind "documentation") (name "") (range (start (line 11) (character 1)) (end (line 11) (character 510))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::datum"))) (kind "attribute") (name "datum") (declared-name "datum") (range (start (line 19) (character 8)) (end (line 19) (character 93))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 19) (character 28)) (end (line 19) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind "part") (name "powerSource") (declared-name "powerSource") (range (start (line 23) (character 2)) (end (line 23) (character 263))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 23) (character 21)) (end (line 23) (character 27)))) (subsetting (reference "componentParts") (range (start (line 23) (character 35)) (end (line 23) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (kind "attribute") (name "ecf") (declared-name "ecf") (range (start (line 24) (character 3)) (end (line 24) (character 207))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "ecf") (range (start (line 24) (character 3)) (end (line 24) (character 10)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (kind "item") (name "shape") (range (start (line 17) (character 8)) (end (line 17) (character 69))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shape") (range (start (line 17) (character 17)) (end (line 17) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::CoordinateFrame"))) (kind "import") (name "CoordinateFrame") (declared-name "CoordinateFrame") (range (start (line 5) (character 1)) (end (line 5) (character 55))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::CoordinateFrame") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 54))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 34) (character 1)) (end (line 34) (character 1583))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SpatialItem") (range (start (line 34) (character 20)) (end (line 34) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::_documentation"))) (kind "documentation") (name "") (range (start (line 34) (character 1)) (end (line 34) (character 1583))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (kind "item") (name "cylinder1") (declared-name "cylinder1") (range (start (line 55) (character 2)) (end (line 55) (character 380))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 60) (character 3)) (end (line 60) (character 226))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 60) (character 17)) (end (line 60) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (kind "item") (name "cylinder2") (declared-name "cylinder2") (range (start (line 69) (character 2)) (end (line 69) (character 428))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (kind "attribute") (name "coordinateFrame") (declared-name "coordinateFrame") (range (start (line 74) (character 3)) (end (line 74) (character 245))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 74) (character 17)) (end (line 74) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing"))) (kind "attribute") (name "cylinderSpacing") (declared-name "cylinderSpacing") (range (start (line 68) (character 2)) (end (line 68) (character 73))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))) (kind "attribute") (name "engineCoordinateFrame") (declared-name "engineCoordinateFrame") (range (start (line 44) (character 2)) (end (line 44) (character 60))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "coordinateFrame") (range (start (line 44) (character 44)) (end (line 44) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (kind "part") (name "rawEngineBlock") (declared-name "rawEngineBlock") (range (start (line 46) (character 2)) (end (line 46) (character 174))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "subSpatialParts") (range (start (line 46) (character 25)) (end (line 46) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing"))) (kind "attribute") (name "rearCylinderSpacing") (declared-name "rearCylinderSpacing") (range (start (line 54) (character 2)) (end (line 54) (character 50))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (kind "item") (name "shape") (range (start (line 42) (character 2)) (end (line 42) (character 21))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shape") (range (start (line 42) (character 11)) (end (line 42) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Point"))) (kind "import") (name "Point") (declared-name "Point") (range (start (line 3) (character 1)) (end (line 3) (character 31))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Point") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 30))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Rotation"))) (kind "import") (name "Rotation") (declared-name "Rotation") (range (start (line 8) (character 1)) (end (line 8) (character 48))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Rotation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 47))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::Translation"))) (kind "import") (name "Translation") (declared-name "Translation") (range (start (line 7) (character 1)) (end (line 7) (character 51))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::Translation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 50))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::TranslationRotationSequence"))) (kind "import") (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence") (range (start (line 6) (character 1)) (end (line 6) (character 67))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::TranslationRotationSequence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 66))))))
    (element (id (node (document "d0") (qualified-name "CarWithShapeAndCSG::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 4) (character 1)) (end (line 4) (character 48))) (parent (node (document "d0") (qualified-name "CarWithShapeAndCSG"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 47))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SpatialItems::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ShapeItems::*") (range (start (line 2) (character 16)) (end (line 2) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 9) (character 16)) (end (line 9) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 11) (character 17)) (end (line 11) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::datum"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 19) (character 28)) (end (line 19) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 23) (character 21)) (end (line 23) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource"))) (kind subsetting) (ordinal 0)) (authored-target "componentParts") (range (start (line 23) (character 35)) (end (line 23) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (kind redefinition) (ordinal 0)) (authored-target "ecf") (range (start (line 24) (character 3)) (end (line 24) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (kind redefinition) (ordinal 0)) (authored-target "shape") (range (start (line 17) (character 17)) (end (line 17) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::CoordinateFrame"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::CoordinateFrame") (range (start (line 5) (character 16)) (end (line 5) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine"))) (kind specialization) (ordinal 0)) (authored-target "SpatialItem") (range (start (line 34) (character 20)) (end (line 34) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 60) (character 17)) (end (line 60) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 74) (character 17)) (end (line 74) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::engineCoordinateFrame"))) (kind redefinition) (ordinal 0)) (authored-target "coordinateFrame") (range (start (line 44) (character 44)) (end (line 44) (character 59))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rawEngineBlock"))) (kind subsetting) (ordinal 0)) (authored-target "subSpatialParts") (range (start (line 46) (character 25)) (end (line 46) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (kind redefinition) (ordinal 0)) (authored-target "shape") (range (start (line 42) (character 11)) (end (line 42) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Point"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Point") (range (start (line 3) (character 16)) (end (line 3) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Rotation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Rotation") (range (start (line 8) (character 16)) (end (line 8) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Translation"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::Translation") (range (start (line 7) (character 16)) (end (line 7) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::TranslationRotationSequence"))) (kind membershipImport) (ordinal 0)) (authored-target "MeasurementReferences::TranslationRotationSequence") (range (start (line 6) (character 16)) (end (line 6) (character 66))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (range (start (line 4) (character 16)) (end (line 4) (character 47))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::powerSource::ecf"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder1::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinder2::coordinateFrame"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (target (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::shape"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "CarWithShapeAndCSG::Car::shape")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::cylinderSpacing")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "CarWithShapeAndCSG::Engine::rearCylinderSpacing")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
