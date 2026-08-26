# META
~~~ini
description=KerML Type ownedFeature derives direct Feature-membership member elements from canonical facts
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeature
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  type Container {
    feature owned;
  }
  type Empty;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-element
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeature")
    (source "Model::Container")
    (target "Model::Container::owned")
    (outcome resolved))
  (type-derived-element
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeature")
    (source "Model::Empty")
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_feature.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e38d02223f77e3427c3deccddec9133a2955ea3adc82ea6c95b58de027fcaf6b") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature.md") (qualified-name "Model::Container"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature.md") (qualified-name "Model::Container::owned"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature.md") (qualified-name "Model::Empty"))) (kind kerml-type) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_feature.md") (qualified-name "Model::Container::owned"))) (target (node (document "memory://snapshot/kerml_type_owned_feature.md") (qualified-name "Model::Container"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature.md") (qualified-name "Model::Container::owned")))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_feature.md") (qualified-name "Model::Container")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
