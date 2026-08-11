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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a5a71058d2d0ba6cd7443c11b54d9b57a7a205c31258a8dc2e0003d8655f60b3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (kind "package") (name "ExternalShapeRefExample") (declared-name "ExternalShapeRefExample"))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ShapeItems::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))) (kind "metadata def") (name "ExternalShapeRef") (declared-name "ExternalShapeRef") (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (kind "attribute") (name "purpose") (declared-name "purpose") (parent (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (kind "attribute") (name "shapeIri") (declared-name "shapeIri") (parent (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::mass"))) (kind "import") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::mass") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::mm"))) (kind "import") (name "mm") (declared-name "mm") (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::mm") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ExternalShapeRefExample::myBatteryUnit"))) (kind "part") (name "myBatteryUnit") (declared-name "myBatteryUnit") (parent (node (document "d0") (qualified-name "ExternalShapeRefExample"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ShapeItems::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ExternalShapeRefExample::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ExternalShapeRefExample::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::mass"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ExternalShapeRefExample::mm"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::mm") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (target (node (document "d0") (qualified-name "ExternalShapeRefExample::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::purpose"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (target (node (document "d0") (qualified-name "ExternalShapeRefExample::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ExternalShapeRefExample::ExternalShapeRef::shapeIri"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 16) (end 4 22)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "ExternalShapeRefExample::mm"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::mm")
        (range (start 4 16) (end 4 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 25)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "ExternalShapeRefExample::mass"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 3 16) (end 3 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 26)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "ExternalShapeRefExample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ShapeItems::*")
        (range (start 2 16) (end 2 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 36)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "ExternalShapeRefExample::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 1 16) (end 1 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
