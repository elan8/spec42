# META
~~~ini
description=KerML 8.3.4.12.3 validateMetadataFeatureAnnotatedElement requires the annotatedElements of a MetadataFeature to have an abstract syntax metaclass consistent with the metaclass annotatedElement declarations
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.12.3 validateMetadataFeatureAnnotatedElement
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.12.3:validateMetadataFeatureAnnotatedElement
blocked_by=lowering-metadata-feature-facts
type=file
~~~
# SOURCE
~~~kerml
package Metadata {
    classifier Thing;
    feature loose : Thing;
    metaclass ClassifierMarker;

    // Conforming: the annotated element is a classifier.
    metadata onClassifier : ClassifierMarker about Thing;

    // Invalid: the annotated element is a feature, which ClassifierMarker does not admit.
    metadata onFeature : ClassifierMarker about loose;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_feature_annotated_element.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "metadata_annotated_element_incompatible")
        (source "semantic")
        (range (start 9 4) (end 9 54))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_feature_annotated_element.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6440cb14b7acb8433a5ac3df4b28593dbdf567b5142271e4282c80c873262200") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClassifierMarker")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClassifierMarker")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassifierMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassifierMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 2 20) (end 2 25)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 6 28) (end 6 44)) (probe (position 6 28))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind featureTyping) (ordinal 0) (authored-target "ClassifierMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 9 25) (end 9 41)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind featureTyping) (ordinal 0) (authored-target "ClassifierMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))))
    )
  )
)
~~~
