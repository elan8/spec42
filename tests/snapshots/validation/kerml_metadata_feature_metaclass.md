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
blocked_by=semantic-metadata-type-not-metaclass
type=file
~~~
# SOURCE
~~~kerml
package Metadata {
    classifier Thing;
    metaclass Marker;

    // Conforming: the metadata feature is typed by a metaclass.
    metadata Marker about Thing;

    // Invalid: the metadata feature is typed by a classifier that is not a metaclass.
    metadata Thing about Thing;
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
        (range (start 8 4) (end 8 31))
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7ed33eb772185577294e50d6621a7443f090c7021b701d0f81120e7b51d63438") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (qualified-name "Metadata"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "Marker"))))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "Marker"))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-classifier) (name "Thing"))))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "Thing"))))) (kind metadata) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
