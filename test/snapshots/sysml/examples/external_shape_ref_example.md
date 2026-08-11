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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "external_shape_ref_example.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a5a71058d2d0ba6cd7443c11b54d9b57a7a205c31258a8dc2e0003d8655f60b3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (kind "package") (name "ExternalShapeRefExample") (declared-name "ExternalShapeRefExample") (range (start (line 0) (character 0)) (end (line 0) (character 684))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 30))) (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ShapeItems::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))) (kind "metadata def") (name "ExternalShapeRef") (declared-name "ExternalShapeRef") (range (start (line 6) (character 1)) (end (line 6) (character 177))) (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::_documentation"))) (kind "documentation") (name "") (range (start (line 6) (character 1)) (end (line 6) (character 177))) (parent (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (kind "attribute") (name "purpose") (declared-name "purpose") (range (start (line 12) (character 2)) (end (line 12) (character 32))) (parent (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (kind "attribute") (name "shapeIri") (declared-name "shapeIri") (range (start (line 13) (character 2)) (end (line 13) (character 33))) (parent (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::mass"))) (kind "import") (name "mass") (declared-name "mass") (range (start (line 3) (character 1)) (end (line 3) (character 26))) (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::mass") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 25))))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::mm"))) (kind "import") (name "mm") (declared-name "mm") (range (start (line 4) (character 1)) (end (line 4) (character 23))) (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::mm") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 22))))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::myBatteryUnit"))) (kind "part") (name "myBatteryUnit") (declared-name "myBatteryUnit") (range (start (line 16) (character 1)) (end (line 16) (character 347))) (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ShapeItems::*") (range (start (line 2) (character 16)) (end (line 2) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ExternalShapeRefExample::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ExternalShapeRefExample::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::mass"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 3) (character 16)) (end (line 3) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::mm"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::mm") (range (start (line 4) (character 16)) (end (line 4) (character 22))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (target (node (document "d0") (qualified-name "ExternalShapeRefExample::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (target (node (document "d0") (qualified-name "ExternalShapeRefExample::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
