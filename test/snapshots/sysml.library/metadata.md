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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/metadata.md"
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
        (range (start 12 39) (end 12 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 51) (end 12 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 19 3) (end 19 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 53) (end 22 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 66) (end 22 71))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:35078522e6e2f1bce94728d1620219b7bd4fd4365a8eb2502bfd5e3a6be4bdfb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::Metaobject") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::metaobjects") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Items::Item") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Items::items") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::MetadataItem"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobject")) (specialization (reference "Item"))))
    (declaration (id (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::metadataItems"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "metaobjects")) (specialization (reference "items"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::Metaobject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::metaobjects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Items::Item")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Items::items")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::MetadataItem"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::MetadataItem"))) (kind specialization) (ordinal 1))
      (authored-target "Item")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::metadataItems"))) (kind specialization) (ordinal 0))
      (authored-target "metaobjects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::metadataItems"))) (kind specialization) (ordinal 1))
      (authored-target "items")
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
  (query (document "memory://snapshot/metadata.md") (range (start 7 16) (end 7 39)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::Metaobject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata.md") (range (start 8 16) (end 8 40)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::metaobjects")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata.md") (range (start 9 16) (end 9 27)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Items::Item")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata.md") (range (start 10 16) (end 10 28)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Items::items")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata.md") (range (start 12 39) (end 12 49)) (probe (position 12 39))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::MetadataItem"))) (kind specialization) (ordinal 0) (authored-target "Metaobject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata.md") (range (start 12 51) (end 12 55)) (probe (position 12 51))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::MetadataItem"))) (kind specialization) (ordinal 1) (authored-target "Item")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata.md") (range (start 22 53) (end 22 64)) (probe (position 22 53))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::metadataItems"))) (kind specialization) (ordinal 0) (authored-target "metaobjects")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata.md") (range (start 22 66) (end 22 71)) (probe (position 22 66))
    (reference (id (source (node (document "memory://snapshot/metadata.md") (qualified-name "Metadata::metadataItems"))) (kind specialization) (ordinal 1) (authored-target "items")
      (outcome (status unresolved)))
  )
)
~~~
