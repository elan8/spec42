# META
~~~ini
description=KerML KerML Spec Annex A: A-2-Atoms
type=file
~~~
# SOURCE
~~~kerml
package Atoms {
	doc
	/* This package defines a keyword (atom) for classifiers with
	 * exactly one instance and are disjoint from any others
	 * marked with this keyword.
	 */

	private import Metaobjects::Metaobject;
	
	classifier Atom;
	metaclass <atom> AtomMetadata specializes Metaobject {
		baseType = Atom meta KerML::Classifier;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_2_atoms.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 39))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package Atoms {
	doc
	/* This package defines a keyword (atom) for classifiers with
	 * exactly one instance and are disjoint from any others
	 * marked with this keyword.
	 */

	private import Metaobjects::Metaobject;
	
	classifier Atom;
	metaclass <atom> AtomMetadata specializes Metaobject {
		baseType = Atom meta KerML::Classifier;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1484a0dae521515b571fb71fe10b62d6b07738481eef110c8a5b012d494bc8f3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Atoms"))) (kind "package") (name "Atoms") (declared-name "Atoms") (range (start (line 0) (character 0)) (end (line 0) (character 341))))
    (element (id (node (document "d0") (qualified-name "Atoms::Atom"))) (kind "classifier decl") (name "Atom") (declared-name "Atom") (range (start (line 9) (character 1)) (end (line 9) (character 17))) (parent (node (document "d0") (qualified-name "Atoms"))))
    (element (id (node (document "d0") (qualified-name "Atoms::Metaobject"))) (kind "import") (name "Metaobject") (declared-name "Metaobject") (range (start (line 7) (character 1)) (end (line 7) (character 40))) (parent (node (document "d0") (qualified-name "Atoms"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::Metaobject") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Atoms::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 341))) (parent (node (document "d0") (qualified-name "Atoms"))))
    (element (id (node (document "d0") (qualified-name "Atoms::atom"))) (kind "kermlDecl") (name "atom") (declared-name "atom") (range (start (line 10) (character 1)) (end (line 10) (character 100))) (parent (node (document "d0") (qualified-name "Atoms"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Atoms::Metaobject"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::Metaobject") (range (start (line 7) (character 16)) (end (line 7) (character 39))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
