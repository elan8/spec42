# META
~~~ini
description=KerML Type ownedFeatureMembership retains the owned FeatureMembership rather than reconstructing it from member names
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeatureMembership
blocked_by=lowering-gap-type-feature-membership-identity
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  type Container {
    feature owned;
  }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOwnedFeatureMembership")
    (source "Model::Container")
    (target "Model::Container::owned")
    (outcome resolved)))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:13079bfea6eb97c9b094f0e205539128f8c677f6881fc15e23c43683cdd9be26") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container::owned"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container::owned"))) (target (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container::owned")))
      (featured-by (node (document "memory://snapshot/kerml_type_owned_feature_membership.md") (qualified-name "Model::Container")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
