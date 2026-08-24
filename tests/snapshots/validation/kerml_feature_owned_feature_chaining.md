# META
~~~ini
description=KerML deriveFeatureOwnedFeatureChaining projects authored chaining facts, including unresolved targets
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 deriveFeatureOwnedFeatureChaining
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining
libraries=none
type=file
~~~
# SOURCE
~~~kerml
package Model {
    classifier Vehicle {
        feature base;
        feature derived chains base;
        feature unresolved chains Missing;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining")
    (source "Model::Vehicle::derived")
    (kind feature_chaining)
    (target "Model::Vehicle::base")
    (provenance authored)
    (outcome resolved))
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining")
    (source "Model::Vehicle::unresolved")
    (kind feature_chaining)
    (provenance authored)
    (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_feature_chaining.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 34) (end 4 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0ec65d8a0810f87a8743fce2f74facff0cc9545c6db88c9f24e33972461f08aa") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::derived"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureChaining (reference "base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::unresolved"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureChaining (reference "Missing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::derived"))) (kind featureChaining) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::unresolved"))) (kind featureChaining) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::derived"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::base"))) (target (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::unresolved"))) (target (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::base")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::derived")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::unresolved")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (range (start 3 31) (end 3 35)) (probe (position 3 31))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::derived"))) (kind featureChaining) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (range (start 4 34) (end 4 41)) (probe (position 4 34))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_feature_chaining.md") (qualified-name "Model::Vehicle::unresolved"))) (kind featureChaining) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
