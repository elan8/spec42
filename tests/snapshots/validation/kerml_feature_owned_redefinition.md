# META
~~~ini
description=KerML deriveFeatureOwnedRedefinition projects an authored redefinition from canonical relationships
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 deriveFeatureOwnedRedefinition
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedRedefinition
libraries=none
type=file
~~~
# SOURCE
~~~kerml
package Model {
    classifier Vehicle {
        feature base;
        feature derived redefines base;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedRedefinition")
    (source "Model::Vehicle::derived")
    (kind redefinition)
    (target "Model::Vehicle::base")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_redefinition.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "redefinition_featuring_type_incompatible")
        (source "semantic")
        (range (start 3 34) (end 3 38))
        (related-information
          (related
            (uri "memory://snapshot/kerml_feature_owned_redefinition.md")
            (range (start 2 8) (end 2 21))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:825cdf2ff4a459a7bc8a4d829b630003552391e0f3ad1009aa9a1dc3a9e7b5e6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::derived"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::base")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::base"))) (target (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::base")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::derived")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::derived")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle")))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::base")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_redefinition.md") (range (start 3 34) (end 3 38)) (probe (position 3 34))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_redefinition.md") (qualified-name "Model::Vehicle::base")))))
    )
  )
)
~~~
