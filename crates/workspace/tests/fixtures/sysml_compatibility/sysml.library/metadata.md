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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Metadata"))) (name "Metadata") (declared-name "Metadata")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Metadata::Item"))) (name "Item") (declared-name "Item"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (name "MetadataItem") (declared-name "MetadataItem")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Metadata::MetadataItem::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Metadata::MetadataItem")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Metadata::Metaobject"))) (name "Metaobject") (declared-name "Metaobject"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Metadata::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "Metadata::items"))) (name "items") (declared-name "items"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Metadata::metadataItems"))) (name "metadataItems") (declared-name "metadataItems")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Metadata::metadataItems::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Metadata::metadataItems")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Metadata::metaobjects"))) (name "metaobjects") (declared-name "metaobjects"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Metadata::MetadataItem::_documentation"))) (to (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Metadata::_documentation"))) (to (node (document "d0") (qualified-name "Metadata"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Metadata::metadataItems::_documentation"))) (to (node (document "d0") (qualified-name "Metadata::metadataItems"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Metadata::metadataItems"))) (status missing-prerequisite) (target "Items::Item"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/metadata.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 1) (end 12 293))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 1) (end 12 293))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 1) (end 22 270))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 1) (end 22 270))
      )
    )
  )
)
~~~
