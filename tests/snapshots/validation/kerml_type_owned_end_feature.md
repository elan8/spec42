# META
~~~ini
description=KerML Type ownedEndFeature filters canonical direct Feature members by their authored end modifier
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedEndFeature
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  type Container {
    feature ordinary;
    end feature endpoint;
  }
  type Empty;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-element
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedEndFeature")
    (source "Model::Container")
    (target "Model::Container::endpoint")
    (outcome resolved))
  (type-derived-element
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedEndFeature")
    (source "Model::Empty")
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_end_feature.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e74fd8ddc7f9141b9dc40ab76f8de28482a08dbcc4af14ad78fa857fd74a36b4") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container::endpoint"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container::ordinary"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Empty"))) (kind kerml-type) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container::endpoint"))) (target (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container::ordinary"))) (target (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container::endpoint")))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container::ordinary")))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_end_feature.md") (qualified-name "Model::Container")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
