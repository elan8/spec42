# META
~~~ini
description=KerML 8.3.4.12.3 validateMetadataFeatureBody requires every ownedFeature of a MetadataFeature to have no declared name, to redefine a single Feature, and to have either no featureValue or a model-level evaluable one, recursively
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.12.3 validateMetadataFeatureBody
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.12.3:validateMetadataFeatureBody
blocked_by=parser-gap-70-metadata-feature-body-member
type=file
~~~
# SOURCE
~~~kerml
package Metadata {
    classifier Thing;
    metaclass Marker;

    // Conforming: the metadata feature owns no body member of its own.
    metadata plain : Marker about Thing;

    // Invalid: the body member declares a name and redefines nothing.
    metadata named : Marker about Thing {
        feature label : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_feature_body.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "metadata_body_feature_invalid")
        (source "semantic")
        (range (start 9 8) (end 9 30))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_metadata_feature_body.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 9 8) (end 10 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:8afbe19b8ad7a430b0098e814bd52b017bc6054afcc85a81ef775e3035d85ace") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::named"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Marker")))))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::plain"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Marker")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::named"))) (kind featureTyping) (ordinal 0))
      (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")))))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::named"))) (target (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::named"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::plain"))) (target (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::plain"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::named")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::plain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::named")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::plain")))
      (type (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_metadata_feature_body.md") (range (start 8 21) (end 8 27)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::named"))) (kind featureTyping) (ordinal 0) (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")))))
    )
  )
  (query (document "memory://snapshot/kerml_metadata_feature_body.md") (range (start 5 21) (end 5 27)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Marker")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_metadata_feature_body.md") (qualified-name "Metadata::Marker")))))
    )
  )
)
~~~
