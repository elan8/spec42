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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 7) (end 11 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 7) (end 12 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 7) (end 13 13))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:67f7c47c3bf4f8cd6cba39ecfaf2a4930cbd5c427ebc17de07ad223eba41c0f6") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ShapeItems::Box") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::mm") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Example car with simple enveloping shape that is a solid box\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Box")) (subsetting (reference "boundingShapes")))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0))))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "length")))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1))))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "width")))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2))))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "height")))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
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
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "width")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "height")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (target (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1))))) (target (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2))))) (target (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 4800))) (unit "mm")))
    (evaluated (declaration (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 1840))) (unit "mm")))
    (evaluated (declaration (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 1350))) (unit "mm")))
    (unit (declaration (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "mm") (start 11 22) (end 11 24) (outcome (status catalog-unavailable)))
    (unit (declaration (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "mm") (start 12 22) (end 12 24) (outcome (status catalog-unavailable)))
    (unit (declaration (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "mm") (start 13 22) (end 13 24) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox")))
      (featured-by (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/car_with_enveloping_shape.md") (qualified-name "CarWithEnvelopingShape::Car::boundingBox")))
    )
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)))))
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
  (query (document "memory://snapshot/car_with_enveloping_shape.md") (range (start 11 7) (end 11 13)) (probe (position 11 7))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_enveloping_shape.md") (range (start 12 7) (end 12 12)) (probe (position 12 7))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "width")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/car_with_enveloping_shape.md") (range (start 13 7) (end 13 13)) (probe (position 13 7))
    (reference (id (source (node (document "memory://snapshot/car_with_enveloping_shape.md") (path (named (kind package) (name "CarWithEnvelopingShape")) (named (kind part-def) (name "Car")) (named (kind item) (name "boundingBox")) (anonymous (kind default-reference) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "height")
      (outcome (status unresolved)))
    )
  )
)
~~~
