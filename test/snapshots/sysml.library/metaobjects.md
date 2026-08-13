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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 10 4) (end 24 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 4) (end 24 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 26 4) (end 43 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 4) (end 43 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 45 4) (end 50 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 4) (end 50 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:17fdd9335fdd59579b9677b75a3df83454d2a5c529007cc2ceaba22762f091a7") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (qualified-name "Metaobjects"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::Object") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Objects::objects") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "KerML::Element") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "KerML::Type") (import (shape membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Objects::objects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "KerML::Element")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "KerML::Type")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/metaobjects.md") (range (start 5 19) (end 5 34)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 6 19) (end 6 35)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Objects::objects")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 7 19) (end 7 33)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "KerML::Element")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metaobjects.md") (range (start 8 19) (end 8 30)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/metaobjects.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "KerML::Type")
      (outcome (status unresolved)))
  )
)
~~~
