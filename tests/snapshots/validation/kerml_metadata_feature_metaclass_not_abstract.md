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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ddcb5fcbf4b5211be071ec2b41bd0a8e98e1877bfded0bef84fe53cea2982524") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "AbstractMarker"))))) (kind kerml-metaclass) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "AbstractMarker"))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind kerml-metaclass) (name "Marker"))))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (path (named (kind package) (name "Metadata")) (named (kind metadata) (name "Marker"))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_metadata_feature_metaclass_not_abstract.md") (qualified-name "Metadata::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
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
