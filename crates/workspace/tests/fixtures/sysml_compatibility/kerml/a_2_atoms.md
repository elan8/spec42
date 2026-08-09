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
    doc /* This package defines a keyword (atom) for classifiers with
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
(model
  (namespace
    (package 'Atoms'
      (documentation)
      (membership_import private -> 'Metaobjects::Metaobject'[unresolved])
      (classifier_def 'Atom')
      (metaclass_def 'AtomMetadata' :> 'Metaobject'[unresolved]
        (feature_def 'baseType'
          (feature_value (=)))))))
~~~
