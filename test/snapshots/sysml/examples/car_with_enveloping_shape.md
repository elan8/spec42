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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1be731a1f518f56721b7f0f9d847d3d8e18dc21688beb35022611e4336435ed6") (contract-version "canonical-resolution-v1"))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 22)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "CarWithEnvelopingShape::mm"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::mm")
        (range (start 2 16) (end 2 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 3) (end 12 12)) (probe (position 12 3))
      (reference
        (source (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width"))
        (kind redefinition) (ordinal 0) (authored-target "width")
        (range (start 12 3) (end 12 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::width") (range (start 12 3) (end 12 26)))
        )
      )
    )
    (query (range (start 11 3) (end 11 13)) (probe (position 11 3))
      (reference
        (source (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length"))
        (kind redefinition) (ordinal 0) (authored-target "length")
        (range (start 11 3) (end 11 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::length") (range (start 11 3) (end 11 26)))
        )
      )
    )
    (query (range (start 13 3) (end 13 13)) (probe (position 13 3))
      (reference
        (source (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height"))
        (kind redefinition) (ordinal 0) (authored-target "height")
        (range (start 13 3) (end 13 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CarWithEnvelopingShape::Car::boundingBox::height") (range (start 13 3) (end 13 26)))
        )
      )
    )
    (query (range (start 1 16) (end 1 31)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "CarWithEnvelopingShape::Box"))
        (kind membershipImport) (ordinal 0) (authored-target "ShapeItems::Box")
        (range (start 1 16) (end 1 31))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
