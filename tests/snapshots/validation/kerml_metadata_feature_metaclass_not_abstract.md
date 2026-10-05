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
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Marker")) (metadataAnnotationAbout (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1))))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AbstractMarker")) (metadataAnnotationAbout (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::AbstractMarker"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Marker"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Marker")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "AbstractMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::AbstractMarker")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1))))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Marker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::AbstractMarker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1))))) (kind metadataAnnotationAbout) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Marker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Marker")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Marker")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1)))))
      (type (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::AbstractMarker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::AbstractMarker")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::AbstractMarker")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::AbstractMarker")))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Marker")))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0)))) (scopes any))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (form usage) (definition (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Marker")))) (about (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
  (annotation (element (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (form usage) (definition (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::AbstractMarker")))) (about (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (range (start 6 13) (end 6 19)) (probe (position 6 13))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Marker")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (range (start 9 13) (end 9 27)) (probe (position 9 13))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "AbstractMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::AbstractMarker")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (range (start 6 26) (end 6 31)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (range (start 9 34) (end 9 39)) (probe (position 9 34))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (anonymous (kind metadata) (ordinal 1))))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing")))))
    )
  )
)
~~~
