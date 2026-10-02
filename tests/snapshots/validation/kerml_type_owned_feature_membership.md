# META
~~~ini
description=KerML Type ownedFeatureMembership retains the owned FeatureMembership rather than reconstructing it from member names
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeatureMembership
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  type Container {
    feature owned;
  }
  type Inheritor specializes Container;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeatureMembership")
    (source "Model::Container")
    (target "Model::Container::owned")
    (outcome resolved))
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeatureMembership")
    (source "Model::Inheritor")
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_owned_feature_membership.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:07dbc88414ad85b55dda92db886e245d9a569a88f8a829b7b9aef0a6be37177f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container::owned"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Inheritor"))) (kind kerml-type) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Container")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Inheritor"))) (kind specialization) (ordinal 0))
      (authored-target "Container")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Inheritor"))) (target (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Inheritor"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container::owned"))) (target (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container")))
      (subtype (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Inheritor")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container::owned")))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Inheritor")))
      (supertype (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_owned_feature_membership.md") (range (start 4 29) (end 4 38)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Inheritor"))) (kind specialization) (ordinal 0) (authored-target "Container")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container")))))
    )
  )
)
~~~
