# META
~~~ini
description=KerML 8.3.4.12.3 deriveMetadataFeatureMetaclass returns an undeclared variable in its pinned body
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.4.12.3:deriveMetadataFeatureMetaclass
blocked_by=normative-specification-gap-metadata-feature-metaclass-body
libraries=standard
type=file
~~~
# SOURCE
~~~kerml
package Metaclasses {
    metaclass Approval;
    classifier Design;
    metadata approved : Approval about Design;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "kerml-1.0:8.3.4.12.3:deriveMetadataFeatureMetaclass") (source "Metaclasses::approved") (target "Metaclasses::Approval") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d4b34b952e8e6f6decc582e8f7dfff18fc402c4596b280f9942aed2b7ec61778") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Design"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (kind kerml-metadata-feature) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Approval")) (metadataAnnotationAbout (reference "Design")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (kind featureTyping) (ordinal 0))
      (authored-target "Approval")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (kind metadataAnnotationAbout) (ordinal 0))
      (authored-target "Design")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Design")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotationAbout) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Design"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (kind metadataAnnotationAbout) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval"))) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (target (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval")) (scopes any))
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
  (annotation (element (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Design"))) (form usage) (definition (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval")))) (about (resolved (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Design")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (range (start 3 24) (end 3 32)) (probe (position 3 24))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (kind featureTyping) (ordinal 0) (authored-target "Approval")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Approval")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (range (start 3 39) (end 3 45)) (probe (position 3 39))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::approved"))) (kind metadataAnnotationAbout) (ordinal 0) (authored-target "Design")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_metaclass_normative_gap.md") (qualified-name "Metaclasses::Design")))))
    )
  )
)
~~~
