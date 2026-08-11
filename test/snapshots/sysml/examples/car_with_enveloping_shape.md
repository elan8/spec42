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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "car_with_enveloping_shape.md"
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
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f5a81d6c2ff18c11006cc0a145d5b535e4db5dd8ea4ba74a9064435bcf104e34") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CarWithEnvelopingShape"))) (kind "package") (name "CarWithEnvelopingShape") (declared-name "CarWithEnvelopingShape") (range (start (line 0) (character 0)) (end (line 0) (character 330))))
    (element (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Box"))) (kind "import") (name "Box") (declared-name "Box") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "CarWithEnvelopingShape"))) (authored (membership (kind Import) (visibility "private") (import (reference "ShapeItems::Box") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 31))))))
    (element (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car"))) (kind "part def") (name "Car") (declared-name "Car") (range (start (line 4) (character 1)) (end (line 4) (character 237))) (parent (node (document "d0") (qualified-name "CarWithEnvelopingShape"))))
    (element (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::_documentation"))) (kind "documentation") (name "") (range (start (line 4) (character 1)) (end (line 4) (character 237))) (parent (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car"))))
    (element (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (kind "item") (name "boundingBox") (declared-name "boundingBox") (range (start (line 10) (character 2)) (end (line 10) (character 133))) (parent (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car"))) (authored (membership (kind Feature)) (relationships (typing (reference "Box") (range none)))))
    (element (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 13) (character 3)) (end (line 13) (character 26))) (parent (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "height") (range (start (line 13) (character 3)) (end (line 13) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length"))) (kind "attribute") (name "length") (declared-name "length") (range (start (line 11) (character 3)) (end (line 11) (character 26))) (parent (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "length") (range (start (line 11) (character 3)) (end (line 11) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width"))) (kind "attribute") (name "width") (declared-name "width") (range (start (line 12) (character 3)) (end (line 12) (character 26))) (parent (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "width") (range (start (line 12) (character 3)) (end (line 12) (character 12)))))))
    (element (id (node (document "d0") (qualified-name "CarWithEnvelopingShape::mm"))) (kind "import") (name "mm") (declared-name "mm") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "CarWithEnvelopingShape"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::mm") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 22))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Box"))) (kind membershipImport) (ordinal 0)) (authored-target "ShapeItems::Box") (range (start (line 1) (character 16)) (end (line 1) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (kind featureTyping) (ordinal 0)) (authored-target "Box") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithEnvelopingShape::Box")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height"))) (kind redefinition) (ordinal 0)) (authored-target "height") (range (start (line 13) (character 3)) (end (line 13) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length"))) (kind redefinition) (ordinal 0)) (authored-target "length") (range (start (line 11) (character 3)) (end (line 11) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width"))) (kind redefinition) (ordinal 0)) (authored-target "width") (range (start (line 12) (character 3)) (end (line 12) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width")))))
    (reference (id (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::mm"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::mm") (range (start (line 2) (character 16)) (end (line 2) (character 22))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (target (node (document "d0") (qualified-name "CarWithEnvelopingShape::Box"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height"))) (target (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length"))) (target (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width"))) (target (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
