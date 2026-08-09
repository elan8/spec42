# META
~~~ini
description=SysML Example (Geometry): ExternalShapeRefExample
type=file
~~~
# SOURCE
~~~sysml
package ExternalShapeRefExample {
	private import ScalarValues::String;
	private import ShapeItems::*;
	private import ISQ::mass;
	private import SI::mm;

	metadata def ExternalShapeRef {
		doc
		/*
		 * Metadata to reference an externally defined shape.
		 */
	
		attribute purpose : String[1];
		attribute shapeIri : String[1];
	}
	
	part myBatteryUnit {
	    item :>> shape : Shell {
			metadata ExternalShapeRef {
				purpose = "highLoD";
				shapeIri = "file:/detailed-geometry/LEMS-250W_BatteryHousing_Example.step";
			}
		}		

		private item envelopingBoxBatteryUnit : Box :> envelopingShapes {
			:>> length = 140[mm];
			:>> width = 148[mm];
			:>> height = 90[mm];
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwItem,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwMetadata,Ident,OpenCurly,
Ident,Eq,StringValue,Semicolon,
Ident,Eq,StringValue,Semicolon,
CloseCurly,
CloseCurly,
KwPrivate,KwItem,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ExternalShapeRefExample'
    (import_decl private 'ScalarValues::String')
    (import_decl private 'ShapeItems::*')
    (import_decl private 'ISQ::mass')
    (import_decl private 'SI::mm')
    (metadata_def 'ExternalShapeRef'
      (documentation)
      (attribute_usage 'purpose' : 'String' multiplicity)
      (attribute_usage 'shapeIri' : 'String' multiplicity))
    (part_usage 'myBatteryUnit'
      (item_usage :>> 'shape' : 'Shell'
        (metadata_feature typed 'ExternalShapeRef'
          (feature_def 'purpose' value)
          (feature_def 'shapeIri' value)))
      (item_usage private 'envelopingBoxBatteryUnit' : 'Box' :> 'envelopingShapes'
        (default_ref_usage :>> 'length' value)
        (default_ref_usage :>> 'width' value)
        (default_ref_usage :>> 'height' value)))))
~~~
# FORMAT
~~~sysml
package ExternalShapeRefExample {
    private import ScalarValues::String;
    private import ShapeItems::*;
    private import ISQ::mass;
    private import SI::mm;

    metadata def ExternalShapeRef {
        doc /*
		 * Metadata to reference an externally defined shape.
		 */

        attribute purpose : String [1];
        attribute shapeIri : String [1];
    }

    part myBatteryUnit {
        item :>> shape : Shell {
            @ExternalShapeRef {
                purpose = "highLoD";
                shapeIri = "file:/detailed-geometry/LEMS-250W_BatteryHousing_Example.step";
            }
        }

        private item envelopingBoxBatteryUnit : Box :> envelopingShapes {
            :>> length = 140[mm];
            :>> width = 148[mm];
            :>> height = 90[mm];
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Shell'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'envelopingShapes'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'shape'
semantic.unresolved_name 'Shell'
semantic.unresolved_name 'Box'
semantic.unresolved_name 'envelopingShapes'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
~~~
# SMG
~~~
(model
  (namespace
    (package 'ExternalShapeRefExample'
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (namespace_import private -> 'ShapeItems'[unresolved])
      (membership_import private -> 'ISQ::mass'[unresolved])
      (membership_import private -> 'SI::mm'[unresolved])
      (metadata_def 'ExternalShapeRef'
        (documentation)
        (attribute_usage composite 'purpose' : 'String'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'shapeIri' : 'String'[unresolved]
          (multiplicity_range [1])))
      (part_usage 'myBatteryUnit'
        (item_usage composite :>> 'shape'[unresolved] : 'Shell'[unresolved]
          (metadata_usage :> 'ExternalShapeRefExample::ExternalShapeRef'[metadata_def]
            (feature_def 'purpose' :>> 'ExternalShapeRefExample::ExternalShapeRef::purpose'[attribute_usage][implied]
              (feature_value (=)))
            (feature_def 'shapeIri' :>> 'ExternalShapeRefExample::ExternalShapeRef::shapeIri'[attribute_usage][implied]
              (feature_value (=)))))
        (item_usage composite 'envelopingBoxBatteryUnit' : 'Box'[unresolved] :> 'envelopingShapes'[unresolved]
          (reference_usage reference :>> 'length'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'width'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'height'[unresolved]
            (feature_value (=))))))))
~~~
