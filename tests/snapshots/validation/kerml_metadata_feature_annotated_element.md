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
libraries=standard
type=file
~~~
# SOURCE
~~~kerml
package Metadata {
    classifier Thing;
    feature loose : Thing;
    // The metaclass restricts what it may annotate by redefining Metaobject::annotatedElement.
    metaclass ClassifierMarker specializes Metaobjects::Metaobject {
        feature redefines annotatedElement : KerML::Classifier;
    }

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
        (range (start 12 4) (end 12 54))
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
      (diagnostic
        (severity warning)
        (code "metadata_annotated_element_incompatible")
        (source "semantic")
        (range (start 12 4) (end 12 54))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3fba4f68fc1b3e06714c99b9000991647d3d959de7903905b9d5547ac76cea68") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Metaobjects::Metaobject")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (effective-identification (name "annotatedElement") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "KerML::Classifier")) (redefinition (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ClassifierMarker")) (metadataAnnotationAbout (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ClassifierMarker")) (metadataAnnotationAbout (reference "loose")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (kind specialization) (ordinal 0))
      (authored-target "Metaobjects::Metaobject")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "KerML::Classifier")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Core::Classifier")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassifierMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassifierMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "loose")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Core::Classifier"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (target (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (target (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))
      (type (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Core::Classifier")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Core::Classifier")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Root::Element")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Core::Classifier")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Core::Type")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Root::Element")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Root::Namespace")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")) (scopes any))
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
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing"))) (form usage) (definition (resolved (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))) (about (resolved (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")))))
  (annotation (element (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (form usage) (definition (resolved (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))) (about (resolved (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 4 43) (end 4 66)) (probe (position 4 43))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker"))) (kind specialization) (ordinal 0) (authored-target "Metaobjects::Metaobject")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 5 45) (end 5 62)) (probe (position 5 45))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "KerML::Classifier")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/ker_ml.md") (qualified-name "KerML::Core::Classifier")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 5 26) (end 5 42)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "ClassifierMarker")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject::annotatedElement")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 2 20) (end 2 25)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 9 28) (end 9 44)) (probe (position 9 28))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind featureTyping) (ordinal 0) (authored-target "ClassifierMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 9 51) (end 9 56)) (probe (position 9 51))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onClassifier"))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 12 25) (end 12 41)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind featureTyping) (ordinal 0) (authored-target "ClassifierMarker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::ClassifierMarker")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (range (start 12 48) (end 12 53)) (probe (position 12 48))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::onFeature"))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "loose")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_annotated_element.md") (qualified-name "Metadata::loose")))))
    )
  )
)
~~~
