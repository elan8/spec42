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
# EXPECTED
~~~
semantic.unresolved_name 'Object'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Element'
semantic.unresolved_name 'Type'
semantic.unresolved_name 'Type'
semantic.unresolved_name 'objects'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Object'
semantic.unresolved_name 'self'
semantic.unresolved_name 'Element'
semantic.unresolved_name 'Type'
semantic.unresolved_name 'Type'
semantic.unresolved_name 'objects'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,RegularComment,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwMetaclass,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,RegularComment,
KwAbstract,KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Metaobjects'
    (documentation)
    (import_decl private 'Objects::Object')
    (import_decl private 'Objects::objects')
    (import_decl private 'KerML::Element')
    (import_decl private 'KerML::Type')
    (metaclass_def abstract 'Metaobject' :> 'Object'
      (documentation)
      (feature_def :>> 'self' : 'Metaobject')
      (feature_def abstract 'annotatedElement' : 'Element' multiplicity
        (documentation)))
    (metaclass_def abstract 'SemanticMetadata' :> 'Metaobject'
      (documentation)
      (feature_def abstract :>> 'annotatedElement' : 'Type' multiplicity
        (documentation))
      (feature_def 'baseType' : 'Type' multiplicity
        (documentation)))
    (feature_def 'metaobjects' : 'Metaobject' multiplicity :> 'objects'
      (comment))))
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
(model
  (namespace
    (library_package 'Metaobjects'
      (documentation)
      (membership_import private -> 'Objects::Object'[unresolved])
      (membership_import private -> 'Objects::objects'[unresolved])
      (membership_import private -> 'KerML::Element'[unresolved])
      (membership_import private -> 'KerML::Type'[unresolved])
      (metaclass_def abstract 'Metaobject' :> 'Object'[unresolved]
        (documentation)
        (feature_def :>> 'self'[unresolved] : 'Metaobjects::Metaobject'[metaclass_def])
        (feature_def abstract 'annotatedElement' : 'Element'[unresolved]
          (multiplicity_range [1..*])
          (documentation)))
      (metaclass_def abstract 'SemanticMetadata' :> 'Metaobjects::Metaobject'[metaclass_def]
        (documentation)
        (feature_def abstract :>> 'Metaobjects::Metaobject::annotatedElement'[feature_def] : 'Type'[unresolved]
          (multiplicity_range [1])
          (documentation))
        (feature_def 'baseType' : 'Type'[unresolved]
          (multiplicity_range [1])
          (documentation)))
      (feature_def 'metaobjects' : 'Metaobjects::Metaobject'[metaclass_def] :> 'objects'[unresolved]
        (multiplicity_range [0..*])))))
~~~
