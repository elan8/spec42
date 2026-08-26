# META
~~~ini
description=KerML deriveFeatureOwnedTypeFeaturing projects the implied featuring type from canonical facts
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 deriveFeatureOwnedTypeFeaturing
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTypeFeaturing
libraries=none
type=file
~~~
# SOURCE
~~~kerml
package Model {
    classifier Vehicle {
        feature base;
        feature derived : Vehicle redefines base chains base;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTypeFeaturing")
    (source "Model::Vehicle::derived")
    (kind type_featuring)
    (target "Model::Vehicle")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:62e4e35f103e2a544c94d7957a161f0e93786351c06dac8c3d7a5f7da608227d"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")) (featureChaining (reference "base")) (redefinition (reference "base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureChaining) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")))
      (type (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (range (start 3 26) (end 3 33)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (range (start 3 56) (end 3 60)) (probe (position 3 56))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureChaining) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (range (start 3 44) (end 3 48)) (probe (position 3 44))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::base")))))
    )
  )
)
~~~
