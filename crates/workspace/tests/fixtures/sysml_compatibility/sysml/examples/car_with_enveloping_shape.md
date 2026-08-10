# META
~~~ini
description=SysML Example (Geometry): CarWithEnvelopingShape
type=file
~~~
# SOURCE
~~~sysml
package CarWithEnvelopingShape {
	private import ShapeItems::Box;
	private import SI::mm;

	part def Car {
		doc
		/*
		 * Example car with simple enveloping shape that is a solid box
		 */
	
		item boundingBox : Box [1] :> boundingShapes {
			:>> length = 4800 [mm];
			:>> width  = 1840 [mm];
			:>> height = 1350 [mm];
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
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
  (package_def 'CarWithEnvelopingShape'
    (import_decl private 'ShapeItems::Box')
    (import_decl private 'SI::mm')
    (part_def 'Car'
      (documentation)
      (item_usage 'boundingBox' : 'Box' :> 'boundingShapes' multiplicity
        (default_ref_usage :>> 'length' value)
        (default_ref_usage :>> 'width' value)
        (default_ref_usage :>> 'height' value)))))
~~~
# FORMAT
~~~sysml
package CarWithEnvelopingShape {
    private import ShapeItems::Box;
    private import SI::mm;

    part def Car {
        doc
        /*
		 * Example car with simple enveloping shape that is a solid box
		 */

        item boundingBox : Box [1] :> boundingShapes {
            :>> length = 4800 [mm];
            :>> width  = 1840 [mm];
            :>> height = 1350 [mm];
        }
    }
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'Box'
semantic.unresolved_name 'boundingShapes'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Box'
semantic.unresolved_name 'boundingShapes'
semantic.unresolved_name 'length'
semantic.unresolved_name 'width'
semantic.unresolved_name 'height'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "CarWithEnvelopingShape"))) (name "CarWithEnvelopingShape") (declared-name "CarWithEnvelopingShape")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Box"))) (name "Box") (declared-name "Box"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car"))) (name "Car") (declared-name "Car") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car")))))
            (element (kind "item") (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (name "boundingBox") (declared-name "boundingBox") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height"))) (name "height") (declared-name "height") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length"))) (name "length") (declared-name "length") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width"))) (name "width") (declared-name "width") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::mm"))) (name "mm") (declared-name "mm"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::_documentation"))) (to (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/car_with_enveloping_shape.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 2) (end 10 133))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 11 3) (end 11 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 11 3) (end 11 26))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 12 3) (end 12 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 12 3) (end 12 26))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 13 3) (end 13 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 13 3) (end 13 26))
      )
    )
  )
)
~~~
