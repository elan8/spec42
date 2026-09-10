# META
~~~ini
description=KerML 8.3.4.12.3 validateMetadataFeatureMetaclassNotAbstract forbids the metaclass of a MetadataFeature from being abstract
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.12.3 validateMetadataFeatureMetaclassNotAbstract
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.12.3:validateMetadataFeatureMetaclassNotAbstract
blocked_by=semantic-metadata-metaclass-abstract
type=file
~~~
# SOURCE
~~~kerml
package Metadata {
    classifier Thing;
    metaclass Marker;
    abstract metaclass AbstractMarker;

    // Conforming: the metaclass is concrete.
    metadata Marker about Thing;

    // Invalid: the metaclass is abstract.
    metadata AbstractMarker about Thing;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "metadata_metaclass_abstract")
        (source "semantic")
        (range (start 9 4) (end 9 40))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ddcb5fcbf4b5211be071ec2b41bd0a8e98e1877bfded0bef84fe53cea2982524"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "AbstractMarker"))))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "AbstractMarker"))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotationAbout (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "Marker"))))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "Marker"))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotationAbout (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "AbstractMarker"))))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "Marker"))))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
  )
  (relationships
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "AbstractMarker"))))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "AbstractMarker"))))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "Marker"))))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "Marker"))))) (kind metadataAnnotationAbout) (ordinal 0)))
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
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (form usage) (about (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
  (annotation (element (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (form usage) (about (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (range (start 9 34) (end 9 39)) (probe (position 9 34))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "AbstractMarker"))))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (range (start 6 26) (end 6 31)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "Marker"))))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
    )
  )
)
~~~
