# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureFeatureMembershipTypeFeaturing publishes the implied TypeFeaturing fact for a Feature owned through FeatureMembership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureFeatureMembershipTypeFeaturing
libraries=none
type=file
~~~
# SOURCE
~~~kerml
package Model {
    classifier Vehicle {
        feature mass;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source "Model::Vehicle::mass")
    (target "Model::Vehicle")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_feature_membership_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1b839fc1b586cede00604198f24e02e1ccdf8971b6a5232174b042fbbb9c8a7a") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_feature_membership_type_featuring.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_feature_membership_type_featuring.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_feature_membership_type_featuring.md") (qualified-name "Model::Vehicle::mass"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_feature_membership_type_featuring.md") (qualified-name "Model::Vehicle::mass"))) (target (node (document "memory://snapshot/kerml_feature_feature_membership_type_featuring.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_feature_membership_type_featuring.md") (qualified-name "Model::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/kerml_feature_feature_membership_type_featuring.md") (qualified-name "Model::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
