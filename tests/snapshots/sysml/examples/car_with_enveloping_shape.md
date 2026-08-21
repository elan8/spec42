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
  (document "memory://snapshot/car_with_enveloping_shape.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
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
        (range (start 10 21) (end 10 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 32) (end 10 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 11 3) (end 11 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 12 3) (end 12 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 13 3) (end 13 26))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:67f7c47c3bf4f8cd6cba39ecfaf2a4930cbd5c427ebc17de07ad223eba41c0f6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ShapeItems::Box") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::mm") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Example car with simple enveloping shape that is a solid box\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Box")) (subsetting (reference "boundingShapes")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ShapeItems::Box")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::mm")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (kind featureTyping) (ordinal 0))
      (authored-target "Box")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (kind subsetting) (ordinal 0))
      (authored-target "boundingShapes")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox")))
      (featured-by (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/car_with_enveloping_shape.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ShapeItems::Box")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_enveloping_shape.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SI::mm")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_enveloping_shape.md") (range (start 10 21) (end 10 24)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (kind featureTyping) (ordinal 0) (authored-target "Box")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_enveloping_shape.md") (range (start 10 32) (end 10 46)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (kind subsetting) (ordinal 0) (authored-target "boundingShapes")
      (outcome (status unresolved)))
    )
  )
)
~~~
