# META
~~~ini
description=KerML 8.3.4.12.3 validateMetadataFeatureMetaclass requires a MetadataFeature to have exactly one type that is a Metaclass
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.12.3 validateMetadataFeatureMetaclass
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.12.3:validateMetadataFeatureMetaclass
type=file
~~~
# SOURCE
~~~kerml
package Metadata {
    classifier Thing;
    metaclass Marker;

    // Conforming: the metadata feature is typed by a metaclass.
    metadata marked : Marker about Thing;

    // Invalid: the metadata feature is typed by a classifier that is not a metaclass.
    metadata classified : Thing about Thing;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_feature_metaclass.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "metadata_type_not_metaclass")
        (source "semantic")
        (range (start 8 4) (end 8 44))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_feature_metaclass.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "metadata_type_not_metaclass")
        (source "semantic")
        (range (start 8 4) (end 8 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7a4e2b10140cf0aa80583e5ac57a3debe4058b7d875f0dad5a687b4fb75e7943"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Marker"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified"))) (kind kerml-metadata-feature) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Thing")) (metadataAnnotationAbout (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked"))) (kind kerml-metadata-feature) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Marker")) (metadataAnnotationAbout (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified"))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked"))) (kind featureTyping) (ordinal 0))
      (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Marker")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked"))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified"))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified"))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified"))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked"))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Marker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked"))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked"))) (kind metadataAnnotationAbout) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Marker")))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Marker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Marker")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Marker")) (scopes any))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing"))) (form usage) (definition (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Marker")))) (about (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))))
  (annotation (element (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing"))) (form usage) (definition (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))) (about (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (range (start 8 26) (end 8 31)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (range (start 8 38) (end 8 43)) (probe (position 8 38))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::classified"))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (range (start 5 22) (end 5 28)) (probe (position 5 22))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked"))) (kind featureTyping) (ordinal 0) (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Marker")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (range (start 5 35) (end 5 40)) (probe (position 5 35))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::marked"))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata::Thing")))))
    )
  )
)
~~~
