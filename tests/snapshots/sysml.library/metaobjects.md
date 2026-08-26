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
  (document "memory://snapshot/metaobjects.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 5 19) (end 5 34))
      )
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
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 10 46) (end 10 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 26) (end 16 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 44) (end 18 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 54) (end 32 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 27) (end 38 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 45 47) (end 45 54))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:17fdd9335fdd59579b9677b75a3df83454d2a5c529007cc2ceaba22762f091a7") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines Metaclasses and Features that are related to the typing of syntactic and semantic metadata.\n\t "))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::Object") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::objects") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "KerML::Element") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "KerML::Type") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * A Metaobject contains syntactic or semantic information about one or more annotatedElements. \n\t\t * It is the most general Metaclass. All other Metaclasses must subclassify it directly or indirectly.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Object")))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Metaobject")) (redefinition (reference "self")))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 1) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The Elements annotated by this Metaobject. This is set automatically when a Metaobject is\n\t\t\t * instantiated as the value of a MetadataFeature.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Element")))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * SemanticMetadata is a Metaobject that requires its single annotatedType to directly or indirectly specialize \n\t\t * a baseType that models the semantics for the annotatedType.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobject")))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t\t * The single annotatedElement of this SemanticMetadata, which must be a Type.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Type")) (redefinition (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata::baseType"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t\t * The required base Type for the annotatedType.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Type")))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (comment (text "\n\t\t * metaobjects is a specialization of objects restricted to type Metadata. It is the most general \n\t\t * MetadataFeature. All other MetadataFeatures must subset it directly or indirectly.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Metaobject")) (subsetting (reference "objects")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::objects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "KerML::Element")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "KerML::Type")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (kind specialization) (ordinal 0))
      (authored-target "Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Metaobject")
      (outcome (status resolved) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "Element")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobject")
      (outcome (status resolved) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Type")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement")))))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata::baseType"))) (kind featureTyping) (ordinal 0))
      (authored-target "Type")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))) (kind featureTyping) (ordinal 0))
      (authored-target "Metaobject")
      (outcome (status resolved) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))) (kind subsetting) (ordinal 0))
      (authored-target "objects")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata"))) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement"))) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata::baseType"))) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))
      (subtype (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::metaobjects")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))
      (type (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (provenance authored))
      (effective-type (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (source direct))
      (supertype (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement")))
      (featured-by (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))
      (subtype (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata")))
      (supertype (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata")))
      (supertype (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata::baseType")))
      (featured-by (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::metaobjects")))
      (type (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (provenance authored))
      (effective-type (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (source direct))
      (supertype (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/metaobjects.md") (range (start 5 19) (end 5 34)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 6 19) (end 6 35)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::objects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 7 19) (end 7 33)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "KerML::Element")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 8 19) (end 8 30)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "KerML::Type")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 10 46) (end 10 52)) (probe (position 10 46))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (kind specialization) (ordinal 0) (authored-target "Object")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 16 33) (end 16 43)) (probe (position 16 33))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Metaobject")
      (outcome (status resolved) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 16 26) (end 16 30)) (probe (position 16 26))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "Metaobject")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 18 44) (end 18 51)) (probe (position 18 44))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "Element")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 26 52) (end 26 62)) (probe (position 26 52))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata"))) (kind specialization) (ordinal 0) (authored-target "Metaobject")
      (outcome (status resolved) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 32 54) (end 32 58)) (probe (position 32 54))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Type")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 32 35) (end 32 51)) (probe (position 32 35))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (path (named (kind library-package) (name "Metaobjects")) (named (kind kerml-metaclass) (name "SemanticMetadata")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement")))))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 38 27) (end 38 31)) (probe (position 38 27))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::SemanticMetadata::baseType"))) (kind featureTyping) (ordinal 0) (authored-target "Type")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 45 26) (end 45 36)) (probe (position 45 26))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))) (kind featureTyping) (ordinal 0) (authored-target "Metaobject")
      (outcome (status resolved) (target (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))))
    )
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 45 47) (end 45 54)) (probe (position 45 47))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))) (kind subsetting) (ordinal 0) (authored-target "objects")
      (outcome (status unresolved)))
    )
  )
)
~~~
