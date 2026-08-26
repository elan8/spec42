# META
~~~ini
description=KerML deriveFeatureOwnedSubsetting includes authored redefinitions as canonical Subsetting facts
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 deriveFeatureOwnedSubsetting
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedSubsetting
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
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedSubsetting")
    (source "Model::Vehicle::derived")
    (kind redefinition)
    (target "Model::Vehicle::base")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_subsetting.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2edd9902787269b2cdcc074f855aa98aba23abdac99f227485da21db5098f183") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::base")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::base"))) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::base")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle")))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::base")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_subsetting.md") (range (start 3 34) (end 3 38)) (probe (position 3 34))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::base")))))
    )
  )
)
~~~
