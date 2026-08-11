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
  (document "metadata.md"
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
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "14a3a29d073aaa821087fa6a86f771f17bdbc3cb0690fa9a21e50cf8f6bd77fc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Metadata"))) (kind "package") (name "Metadata") (declared-name "Metadata") (range (start (line 0) (character 0)) (end (line 0) (character 884))))
    (element (id (node (document "d0") (qualified-name "Metadata::Item"))) (kind "import") (name "Item") (declared-name "Item") (range (start (line 9) (character 1)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "Metadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Items::Item") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (kind "metadata def") (name "MetadataItem") (declared-name "MetadataItem") (range (start (line 12) (character 1)) (end (line 12) (character 293))) (parent (node (document "d0") (qualified-name "Metadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Metaobject") (range (start (line 12) (character 39)) (end (line 12) (character 49)))) (specializes (reference "Item") (range (start (line 12) (character 51)) (end (line 12) (character 55)))))))
    (element (id (node (document "d0") (qualified-name "Metadata::MetadataItem::_documentation"))) (kind "documentation") (name "") (range (start (line 12) (character 1)) (end (line 12) (character 293))) (parent (node (document "d0") (qualified-name "Metadata::MetadataItem"))))
    (element (id (node (document "d0") (qualified-name "Metadata::Metaobject"))) (kind "import") (name "Metaobject") (declared-name "Metaobject") (range (start (line 7) (character 1)) (end (line 7) (character 40))) (parent (node (document "d0") (qualified-name "Metadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::Metaobject") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Metadata::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 884))) (parent (node (document "d0") (qualified-name "Metadata"))))
    (element (id (node (document "d0") (qualified-name "Metadata::items"))) (kind "import") (name "items") (declared-name "items") (range (start (line 10) (character 1)) (end (line 10) (character 29))) (parent (node (document "d0") (qualified-name "Metadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Items::items") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Metadata::metadataItems"))) (kind "item def") (name "metadataItems") (declared-name "metadataItems") (range (start (line 22) (character 1)) (end (line 22) (character 270))) (parent (node (document "d0") (qualified-name "Metadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "metaobjects") (range (start (line 0) (character 0)) (end (line 0) (character 11)))) (specializes (reference "items") (range (start (line 0) (character 13)) (end (line 0) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "Metadata::metadataItems::_documentation"))) (kind "documentation") (name "") (range (start (line 22) (character 1)) (end (line 22) (character 270))) (parent (node (document "d0") (qualified-name "Metadata::metadataItems"))))
    (element (id (node (document "d0") (qualified-name "Metadata::metaobjects"))) (kind "import") (name "metaobjects") (declared-name "metaobjects") (range (start (line 8) (character 1)) (end (line 8) (character 41))) (parent (node (document "d0") (qualified-name "Metadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::metaobjects") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 40))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Metadata::Item"))) (kind membershipImport) (ordinal 0)) (authored-target "Items::Item") (range (start (line 9) (character 16)) (end (line 9) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (kind specialization) (ordinal 0)) (authored-target "Metaobject") (range (start (line 12) (character 39)) (end (line 12) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Metadata::Metaobject")))))
    (reference (id (source (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (kind specialization) (ordinal 1)) (authored-target "Item") (range (start (line 12) (character 51)) (end (line 12) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Metadata::Item")))))
    (reference (id (source (node (document "d0") (qualified-name "Metadata::Metaobject"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::Metaobject") (range (start (line 7) (character 16)) (end (line 7) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata::items"))) (kind membershipImport) (ordinal 0)) (authored-target "Items::items") (range (start (line 10) (character 16)) (end (line 10) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata::metadataItems"))) (kind specialization) (ordinal 0)) (authored-target "metaobjects") (range (start (line 0) (character 0)) (end (line 0) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Metadata::metaobjects")))))
    (reference (id (source (node (document "d0") (qualified-name "Metadata::metadataItems"))) (kind specialization) (ordinal 1)) (authored-target "items") (range (start (line 0) (character 13)) (end (line 0) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Metadata::items")))))
    (reference (id (source (node (document "d0") (qualified-name "Metadata::metaobjects"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::metaobjects") (range (start (line 8) (character 16)) (end (line 8) (character 40))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (target (node (document "d0") (qualified-name "Metadata::Item"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (target (node (document "d0") (qualified-name "Metadata::Metaobject"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Metadata::MetadataItem"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Metadata::metadataItems"))) (target (node (document "d0") (qualified-name "Metadata::items"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Metadata::metadataItems"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Metadata::metadataItems"))) (target (node (document "d0") (qualified-name "Metadata::metaobjects"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Metadata::metadataItems"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 12 51) (end 12 55)) (probe (position 12 51))
      (reference
        (source (document "d0") (qualified-name "Metadata::MetadataItem"))
        (kind specialization) (ordinal 1) (authored-target "Item")
        (range (start 12 51) (end 12 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Metadata::Item") (range (start 9 1) (end 9 28)))
        )
      )
    )
    (query (range (start 0 13) (end 0 18)) (probe (position 0 13))
      (reference
        (source (document "d0") (qualified-name "Metadata::metadataItems"))
        (kind specialization) (ordinal 1) (authored-target "items")
        (range (start 0 13) (end 0 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Metadata::items") (range (start 10 1) (end 10 29)))
        )
      )
    )
    (query (range (start 12 39) (end 12 49)) (probe (position 12 39))
      (reference
        (source (document "d0") (qualified-name "Metadata::MetadataItem"))
        (kind specialization) (ordinal 0) (authored-target "Metaobject")
        (range (start 12 39) (end 12 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Metadata::Metaobject") (range (start 7 1) (end 7 40)))
        )
      )
    )
    (query (range (start 0 0) (end 0 11)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "Metadata::metadataItems"))
        (kind specialization) (ordinal 0) (authored-target "metaobjects")
        (range (start 0 0) (end 0 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Metadata::metaobjects") (range (start 8 1) (end 8 41)))
        )
      )
    )
    (query (range (start 9 16) (end 9 27)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Metadata::Item"))
        (kind membershipImport) (ordinal 0) (authored-target "Items::Item")
        (range (start 9 16) (end 9 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 28)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Metadata::items"))
        (kind membershipImport) (ordinal 0) (authored-target "Items::items")
        (range (start 10 16) (end 10 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 39)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Metadata::Metaobject"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::Metaobject")
        (range (start 7 16) (end 7 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 40)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Metadata::metaobjects"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::metaobjects")
        (range (start 8 16) (end 8 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
