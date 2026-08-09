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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (name "ExternalShapeRefExample") (declared-name "ExternalShapeRefExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ExternalShapeRefExample::*"))) (name "*") (declared-name "*"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))) (name "ExternalShapeRef") (declared-name "ExternalShapeRef")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (name "purpose") (declared-name "purpose") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (name "shapeIri") (declared-name "shapeIri") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ExternalShapeRefExample::String"))) (name "String") (declared-name "String"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ExternalShapeRefExample::mass"))) (name "mass") (declared-name "mass"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ExternalShapeRefExample::mm"))) (name "mm") (declared-name "mm"))
        (element (kind "part") (id (node (document "d0") (qualified-name "ExternalShapeRefExample::myBatteryUnit"))) (name "myBatteryUnit") (declared-name "myBatteryUnit") (declared (properties (ordered false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::_documentation"))) (to (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ExternalShapeRefExample::myBatteryUnit"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/external_shape_ref_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
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
        (range (start 3 16) (end 3 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 22))
      )
    )
  )
)
~~~
