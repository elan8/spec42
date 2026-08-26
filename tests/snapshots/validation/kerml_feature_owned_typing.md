# META
~~~ini
description=KerML deriveFeatureOwnedTyping projects an authored FeatureTyping relationship
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 deriveFeatureOwnedTyping
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping
libraries=none
type=file
~~~
# SOURCE
~~~kerml
package Model {
    classifier Mass;
    classifier Vehicle {
        feature mass : Mass;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping")
    (source "Model::Vehicle::mass")
    (kind feature_typing)
    (target "Model::Mass")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_typing.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9ff35eb6410d4d2b2532aca13e541a22c10b785b199fedc32d79d1c4c3e7fb4c") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Mass"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Mass")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Mass")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle::mass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Mass")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle::mass")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle")))
      (type (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Mass")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Mass")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Mass")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_typing.md") (range (start 3 23) (end 3 27)) (probe (position 3 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_typing.md") (qualified-name "Model::Mass")))))
    )
  )
)
~~~
