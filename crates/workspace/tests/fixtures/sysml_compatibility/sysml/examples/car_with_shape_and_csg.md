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
        doc /*
		 * Car with simple engine
		 */

        item :>> shape = new Cuboid(4800 [mm], 1840 [mm], 1350 [mm]);

        attribute datum :>> coordinateFrame {
            :>> mRefs = (mm, mm, mm);
        }

        part powerSource : Engine :> componentParts [1] {
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
        doc /*
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
        private item cylinder1 :> subSpatialParts [1] {
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
        private item cylinder2 :> subSpatialParts [1] {
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
        attribute :> differencesOf [1] {
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
(model
  (namespace
    (package 'CarWithShapeAndCSG'
      (namespace_import private -> 'SpatialItems'[unresolved])
      (namespace_import private -> 'ShapeItems'[unresolved])
      (membership_import private -> 'Objects::Point'[unresolved])
      (membership_import private -> 'Quantities::VectorQuantityValue'[unresolved])
      (membership_import private -> 'MeasurementReferences::CoordinateFrame'[unresolved])
      (membership_import private -> 'MeasurementReferences::TranslationRotationSequence'[unresolved])
      (membership_import private -> 'MeasurementReferences::Translation'[unresolved])
      (membership_import private -> 'MeasurementReferences::Rotation'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (part_def 'Car' :> 'SpatialItem'[unresolved]
        (documentation)
        (item_usage composite :>> 'shape'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'datum' :>> 'coordinateFrame'[unresolved]
          (reference_usage reference :>> 'mRefs'[unresolved]
            (feature_value (=))))
        (part_usage composite 'powerSource' : 'CarWithShapeAndCSG::Engine'[part_def] :> 'componentParts'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'CarWithShapeAndCSG::Engine::engineCoordinateFrame'[attribute_usage]
            (reference_usage reference :>> 'mRefs'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'source'[unresolved]
                (feature_value (=)))
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=)))))))
      (part_def 'Engine' :> 'SpatialItem'[unresolved]
        (documentation)
        (item_usage composite :>> 'shape'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'engineCoordinateFrame' :>> 'coordinateFrame'[unresolved])
        (part_usage composite 'rawEngineBlock' :> 'subSpatialParts'[unresolved]
          (multiplicity_range [1])
          (item_usage composite :>> 'shape'[unresolved] : 'Box'[unresolved]
            (multiplicity_range [1])
            (reference_usage reference :>> 'length'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'width'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'height'[unresolved]
              (feature_value (=)))))
        (attribute_usage composite 'rearCylinderSpacing'
          (feature_value (=)))
        (item_usage composite 'cylinder1' :> 'subSpatialParts'[unresolved]
          (multiplicity_range [1])
          (item_usage composite :>> 'shape'[unresolved] : 'Cylinder'[unresolved]
            (multiplicity_range [1])
            (reference_usage reference :>> 'radius'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'height'[unresolved]
              (feature_value (=))))
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'source'[unresolved]
                (feature_value (=)))
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (attribute_usage composite 'cylinderSpacing'
          (feature_value (=)))
        (item_usage composite 'cylinder2' :> 'subSpatialParts'[unresolved]
          (multiplicity_range [1])
          (item_usage composite :>> 'shape'[unresolved] : 'Cylinder'[unresolved]
            (multiplicity_range [1])
            (reference_usage reference :>> 'radius'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'height'[unresolved]
              (feature_value (=))))
          (attribute_usage composite :>> 'coordinateFrame'[unresolved]
            (reference_usage reference :>> 'transformation'[unresolved] : 'TranslationRotationSequence'[unresolved]
              (reference_usage reference :>> 'source'[unresolved]
                (feature_value (=)))
              (reference_usage reference :>> 'elements'[unresolved]
                (feature_value (=))))))
        (attribute_usage composite :> 'differencesOf'[unresolved]
          (multiplicity_range [1])
          (item_usage composite :>> 'elements'[unresolved]
            (feature_value (=))))))))
~~~
