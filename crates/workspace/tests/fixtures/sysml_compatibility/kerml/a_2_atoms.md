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
# EXPECTED
~~~
semantic.unresolved_name 'Metaobject'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Metaobject'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Atoms"))) (name "Atoms") (declared-name "Atoms")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Atoms::Atom"))) (name "Atom") (declared-name "Atom"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Atoms::Metaobject"))) (name "Metaobject") (declared-name "Metaobject"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Atoms::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Atoms::atom"))) (name "atom") (declared-name "atom"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Atoms::_documentation"))) (to (node (document "d0") (qualified-name "Atoms"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/a_2_atoms.md"
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
