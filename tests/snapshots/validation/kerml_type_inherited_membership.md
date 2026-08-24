# META
~~~ini
description=KerML Type inheritedMembership publishes inherited FeatureMembership facts through canonical specialization closure
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeInheritedMembership
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  type Parent {
    feature inherited;
  }
  type Child specializes Parent;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeInheritedMembership")
    (source "Model::Child")
    (target "Model::Parent::inherited")
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_inherited_membership.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:998040639e4d4a16cb91f217a486e5d2199dfd10d0195890f2f85eacde62b8a6") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Child"))) (kind kerml-type) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent::inherited"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Child"))) (target (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent::inherited"))) (target (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Child")))
      (supertype (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent")))
      (subtype (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent::inherited")))
      (featured-by (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_inherited_membership.md") (range (start 4 25) (end 4 31)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_inherited_membership.md") (qualified-name "Model::Parent")))))
    )
  )
)
~~~
