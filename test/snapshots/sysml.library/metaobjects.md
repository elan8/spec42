# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Metaobjects
type=file
~~~
# SOURCE
~~~kerml
standard library package Metaobjects {
    doc /*
	 * This package defines Metaclasses and Features that are related to the typing of syntactic and semantic metadata.
	 */

    private import Objects::Object;
    private import Objects::objects;
    private import KerML::Element;
    private import KerML::Type;

    abstract metaclass Metaobject specializes Object {
        doc /*
		 * A Metaobject contains syntactic or semantic information about one or more annotatedElements. 
		 * It is the most general Metaclass. All other Metaclasses must subclassify it directly or indirectly.
		 */

        feature redefines self : Metaobject;

        abstract feature annotatedElement : Element [1..*] {
            doc /*
			 * The Elements annotated by this Metaobject. This is set automatically when a Metaobject is
			 * instantiated as the value of a MetadataFeature.
			 */
        }
    }

    abstract metaclass SemanticMetadata specializes Metaobject {
        doc /*
		 * SemanticMetadata is a Metaobject that requires its single annotatedType to directly or indirectly specialize 
		 * a baseType that models the semantics for the annotatedType.
		 */

        abstract feature redefines annotatedElement : Type [1] {
            doc /*
			 * The single annotatedElement of this SemanticMetadata, which must be a Type.
			 */
        }

        feature baseType : Type [1] {
            doc /*
			 * The required base Type for the annotatedType.
			 */
        }
    }

    feature metaobjects : Metaobject [0..*] :> objects {
        /*
		 * metaobjects is a specialization of objects restricted to type Metadata. It is the most general 
		 * MetadataFeature. All other MetadataFeatures must subset it directly or indirectly.
		 */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metaobjects.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 19) (end 5 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 19) (end 6 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 19) (end 7 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 19) (end 8 30))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Metaobjects {
    doc /*
	 * This package defines Metaclasses and Features that are related to the typing of syntactic and semantic metadata.
	 */

    private import Objects::Object;
    private import Objects::objects;
    private import KerML::Element;
    private import KerML::Type;

    abstract metaclass Metaobject specializes Object {
        doc /*
		 * A Metaobject contains syntactic or semantic information about one or more annotatedElements. 
		 * It is the most general Metaclass. All other Metaclasses must subclassify it directly or indirectly.
		 */

        feature redefines self : Metaobject;

        abstract feature annotatedElement : Element [1..*] {
            doc /*
			 * The Elements annotated by this Metaobject. This is set automatically when a Metaobject is
			 * instantiated as the value of a MetadataFeature.
			 */
        }
    }

    abstract metaclass SemanticMetadata specializes Metaobject {
        doc /*
		 * SemanticMetadata is a Metaobject that requires its single annotatedType to directly or indirectly specialize 
		 * a baseType that models the semantics for the annotatedType.
		 */

        abstract feature redefines annotatedElement : Type [1] {
            doc /*
			 * The single annotatedElement of this SemanticMetadata, which must be a Type.
			 */
        }

        feature baseType : Type [1] {
            doc /*
			 * The required base Type for the annotatedType.
			 */
        }
    }

    feature metaobjects : Metaobject [0..*] :> objects {
        /*
		 * metaobjects is a specialization of objects restricted to type Metadata. It is the most general 
		 * MetadataFeature. All other MetadataFeatures must subset it directly or indirectly.
		 */
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d994c3c865ab8b5ec41202e1c53d7fa377b9ea423d3fb2283bbbf494d33f400f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Metaobjects"))) (kind "package") (name "Metaobjects") (declared-name "Metaobjects") (range (start (line 0) (character 0)) (end (line 0) (character 1749))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::Element"))) (kind "import") (name "Element") (declared-name "Element") (range (start (line 7) (character 4)) (end (line 7) (character 34))) (parent (node (document "d0") (qualified-name "Metaobjects"))) (authored (membership (kind Import) (visibility "private") (import (reference "KerML::Element") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 33))))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::Metaobject"))) (kind "kermlDecl") (name "Metaobject") (declared-name "Metaobject") (range (start (line 10) (character 4)) (end (line 10) (character 579))) (parent (node (document "d0") (qualified-name "Metaobjects"))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::Object"))) (kind "import") (name "Object") (declared-name "Object") (range (start (line 5) (character 4)) (end (line 5) (character 35))) (parent (node (document "d0") (qualified-name "Metaobjects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 5) (character 19)) (end (line 5) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::SemanticMetadata"))) (kind "metadata def") (name "SemanticMetadata") (declared-name "SemanticMetadata") (range (start (line 26) (character 4)) (end (line 26) (character 582))) (parent (node (document "d0") (qualified-name "Metaobjects"))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::SemanticMetadata::baseType"))) (kind "attribute def") (name "baseType") (declared-name "baseType") (range (start (line 26) (character 4)) (end (line 26) (character 582))) (parent (node (document "d0") (qualified-name "Metaobjects::SemanticMetadata"))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::SemanticMetadata::redefines"))) (kind "attribute def") (name "redefines") (declared-name "redefines") (range (start (line 26) (character 4)) (end (line 26) (character 582))) (parent (node (document "d0") (qualified-name "Metaobjects::SemanticMetadata"))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::Type"))) (kind "import") (name "Type") (declared-name "Type") (range (start (line 8) (character 4)) (end (line 8) (character 31))) (parent (node (document "d0") (qualified-name "Metaobjects"))) (authored (membership (kind Import) (visibility "private") (import (reference "KerML::Type") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1749))) (parent (node (document "d0") (qualified-name "Metaobjects"))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::metaobjects"))) (kind "feature decl") (name "metaobjects") (declared-name "metaobjects") (range (start (line 45) (character 4)) (end (line 45) (character 268))) (parent (node (document "d0") (qualified-name "Metaobjects"))))
    (element (id (node (document "d0") (qualified-name "Metaobjects::objects"))) (kind "import") (name "objects") (declared-name "objects") (range (start (line 6) (character 4)) (end (line 6) (character 36))) (parent (node (document "d0") (qualified-name "Metaobjects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::objects") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 19)) (end (line 6) (character 35))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Metaobjects::Element"))) (kind membershipImport) (ordinal 0)) (authored-target "KerML::Element") (range (start (line 7) (character 19)) (end (line 7) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metaobjects::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (range (start (line 5) (character 19)) (end (line 5) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metaobjects::Type"))) (kind membershipImport) (ordinal 0)) (authored-target "KerML::Type") (range (start (line 8) (character 19)) (end (line 8) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metaobjects::objects"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::objects") (range (start (line 6) (character 19)) (end (line 6) (character 35))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
