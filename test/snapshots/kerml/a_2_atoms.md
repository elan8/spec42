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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwClassifier,Ident,Semicolon,
KwMetaclass,OpenAngle,Ident,CloseAngle,Ident,KwSpecializes,Ident,OpenCurly,
Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Atoms'
    (documentation)
    (import_decl private 'Metaobjects::Metaobject')
    (classifier_def 'Atom')
    (metaclass_def 'AtomMetadata' :> 'Metaobject'
      (feature_def 'baseType' value))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Metaobject'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Metaobject'
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "bd61becffe00d3eb8e89726a802f2750409dee9b6a75a82cb68292488016304b") (contract-version "canonical-resolution-v1"))
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
