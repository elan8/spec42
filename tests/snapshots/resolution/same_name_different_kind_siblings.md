# META
~~~ini
description=A name shared by siblings of different kinds needs no occurrence ordinal: the kind on every path segment separates them, so `metadata def Safety` and the metadata usage `Safety` typed by it (`metadata Safety : Safety about Vehicle;`) are distinct elements
type=file
libraries=standard
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package P {
    part def Vehicle;
    metadata def Safety;
    metadata Safety : Safety about Vehicle;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/same_name_different_kind_siblings.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:c0b92364fd76cfbe243a76e1c3d859c7d1b374d14eed23bf03e18189de9cffe7") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/same_name_different_kind_siblings.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety"))))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Safety")) (metadataAnnotationAbout (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/same_name_different_kind_siblings.md") (qualified-name "P::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (kind featureTyping) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety")))))))
    (reference (id (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/same_name_different_kind_siblings.md") (qualified-name "P::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (target (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety"))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (target (node (document "memory://snapshot/same_name_different_kind_siblings.md") (qualified-name "P::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety"))))) (target (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (target (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (qualified-name "P::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety")))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety")))))
      (type (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety")))) (provenance authored))
      (effective-type (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety")))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety")))) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/same_name_different_kind_siblings.md") (qualified-name "P::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/same_name_different_kind_siblings.md") (qualified-name "P::Vehicle"))) (form usage) (definition (resolved (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety")))))) (about (resolved (node (document "memory://snapshot/same_name_different_kind_siblings.md") (qualified-name "P::Vehicle")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/same_name_different_kind_siblings.md") (range (start 3 22) (end 3 28)) (probe (position 3 22))
    (reference (id (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (kind featureTyping) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata-def) (name "Safety")))))))
    )
  )
  (query (document "memory://snapshot/same_name_different_kind_siblings.md") (range (start 3 35) (end 3 42)) (probe (position 3 35))
    (reference (id (source (node (document "memory://snapshot/same_name_different_kind_siblings.md") (path (named (kind package) (name "P")) (named (kind metadata) (name "Safety"))))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/same_name_different_kind_siblings.md") (qualified-name "P::Vehicle")))))
    )
  )
)
~~~
