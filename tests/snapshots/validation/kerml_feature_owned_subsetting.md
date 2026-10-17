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
    classifier Base {
        feature base;
    }
    classifier Vehicle specializes Base {
        feature 'derived' redefines base;
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
    (target "Model::Base::base")
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ba3e9d650c27f5ed6b2c5da36c006390b4e35db3271d02e7c582a9fe312496c4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base::base")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle"))) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base::base"))) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base::base")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle")))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle")))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base::base")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_subsetting.md") (range (start 4 35) (end 4 39)) (probe (position 4 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_subsetting.md") (range (start 5 36) (end 5 40)) (probe (position 5 36))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Vehicle::derived"))) (kind redefinition) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_subsetting.md") (qualified-name "Model::Base::base")))))
    )
  )
)
~~~
