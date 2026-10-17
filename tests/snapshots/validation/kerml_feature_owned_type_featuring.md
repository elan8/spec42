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
    classifier Base {
        feature base;
    }
    classifier Vehicle specializes Base {
        feature 'derived' : Vehicle redefines base chains base;
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
      (diagnostic
        (severity warning)
        (code "feature_chaining_single_operand")
        (source "semantic")
        (range (start 5 58) (end 5 62))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b3095bb7dd2dc1d4bb7f4b63327ef983133c47983a4e6a0c7dcfce078258daf0"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")) (featureChaining (reference "base")) (redefinition (reference "base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureChaining) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived")))
      (type (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (range (start 4 35) (end 4 39)) (probe (position 4 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (range (start 5 28) (end 5 35)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (range (start 5 58) (end 5 62)) (probe (position 5 58))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind featureChaining) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (range (start 5 46) (end 5 50)) (probe (position 5 46))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_type_featuring.md") (qualified-name "Model::Base::base")))))
    )
  )
)
~~~
