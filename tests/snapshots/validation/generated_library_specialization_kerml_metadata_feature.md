# META
~~~ini
description=A KerML metadata feature takes the checkMetadataFeatureSpecialization anchor Metaobjects::metaobjects
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.12.3:checkMetadataFeatureSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package GeneratedSpecializationKermlMetadata {
    classifier Thing;
    metaclass Marker;
    metadata MetadataFeature : Marker about Thing;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "GeneratedSpecializationKermlMetadata::MetadataFeature") (target "Metaobjects::metaobjects") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:4b3f8f3aa98ec6d6e87f7a2ac59fae1039b61c3de544df2f7d72c2ff795cf587") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (kind kerml-metadata-feature) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Marker")) (metadataAnnotationAbout (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (kind featureTyping) (ordinal 0))
      (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker")))))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (target (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (target (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker"))) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature")))
      (type (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Thing"))) (form usage) (definition (resolved (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker")))) (about (resolved (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Thing")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (range (start 3 31) (end 3 37)) (probe (position 3 31))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (kind featureTyping) (ordinal 0) (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Marker")))))
    )
  )
  (query (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (range (start 3 44) (end 3 49)) (probe (position 3 44))
    (reference (id (source (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::MetadataFeature"))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_library_specialization_kerml_metadata_feature.md") (qualified-name "GeneratedSpecializationKermlMetadata::Thing")))))
    )
  )
)
~~~
