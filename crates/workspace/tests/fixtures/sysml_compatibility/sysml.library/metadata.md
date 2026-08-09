# META
~~~ini
description=Standard Library: Systems Library/Metadata
type=file
~~~
# SOURCE
~~~sysml
standard library package Metadata {
doc
/*
 * This package defines the base types for metadata definitions and related 
 * metadata annotations in the SysML language.
 */

	private import Metaobjects::Metaobject;
	private import Metaobjects::metaobjects;
	private import Items::Item;
	private import Items::items;
	
	abstract metadata def MetadataItem :> Metaobject, Item {
		doc
		/*
		 * MetadataItem is the most general class of Items that represent Metaobjects. 
		 * MetadataItem is the base type of all MetadataDefinitions.
		 */
		 
		 ref self : MetadataItem redefines Metaobject::self, Item::self;
	}
	
	abstract item metadataItems : MetadataItem[0..*] :> metaobjects, items {
		doc
		/*
		 * metadataItems is the base feature of all MetadataUsages.
		 * 
		 * Note: It is not itself a MetadataUsage, because it is not being used as an
		 * AnnotatingElement here.
		 */
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'Item'
semantic.unresolved_name 'Metaobject::self'
semantic.unresolved_name 'Item::self'
semantic.unresolved_name 'metaobjects'
semantic.unresolved_name 'items'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'Item'
semantic.unresolved_name 'Metaobject::self'
semantic.unresolved_name 'Item::self'
semantic.unresolved_name 'metaobjects'
semantic.unresolved_name 'items'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwMetadata,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Metadata'
    (documentation)
    (import_decl private 'Metaobjects::Metaobject')
    (import_decl private 'Metaobjects::metaobjects')
    (import_decl private 'Items::Item')
    (import_decl private 'Items::items')
    (metadata_def abstract 'MetadataItem' :> 'Metaobject', 'Item'
      (documentation)
      (ref_usage ref 'self' : 'MetadataItem' :>> 'Metaobject::self', 'Item::self'))
    (item_usage abstract 'metadataItems' : 'MetadataItem' :> 'metaobjects', 'items' multiplicity
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Metadata {
    doc /*
 * This package defines the base types for metadata definitions and related 
 * metadata annotations in the SysML language.
 */

    private import Metaobjects::Metaobject;
    private import Metaobjects::metaobjects;
    private import Items::Item;
    private import Items::items;

    abstract metadata def MetadataItem :> Metaobject, Item {
        doc /*
		 * MetadataItem is the most general class of Items that represent Metaobjects. 
		 * MetadataItem is the base type of all MetadataDefinitions.
		 */

        ref self : MetadataItem redefines Metaobject::self, Item::self;
    }

    abstract item metadataItems : MetadataItem :> metaobjects, items [0..*] {
        doc /*
		 * metadataItems is the base feature of all MetadataUsages.
		 * 
		 * Note: It is not itself a MetadataUsage, because it is not being used as an
		 * AnnotatingElement here.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Metadata'
      (documentation)
      (membership_import private -> 'Metaobjects::Metaobject'[unresolved])
      (membership_import private -> 'Metaobjects::metaobjects'[unresolved])
      (membership_import private -> 'Items::Item'[unresolved])
      (membership_import private -> 'Items::items'[unresolved])
      (metadata_def abstract 'MetadataItem' :> 'Metaobject'[unresolved] :> 'Item'[unresolved]
        (documentation)
        (reference_usage reference 'self' : 'Metadata::MetadataItem'[metadata_def] :>> 'Metaobject::self'[unresolved] :>> 'Item::self'[unresolved]))
      (item_usage abstract 'metadataItems' : 'Metadata::MetadataItem'[metadata_def] :> 'metaobjects'[unresolved] :> 'items'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
