# META
~~~ini
description=KerML Type feature-derived collections retain canonical membership and inherited-closure identities
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeFeatureMembership
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeFeature
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeEndFeature
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeDirectedFeature
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeInheritedFeature
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeInput
rule_id=kerml-1.0:8.3.3.1.10:deriveTypeOutput
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
  type Parent {
    feature inherited;
  }
  type Child specializes Parent {
    feature owned;
    end feature endpoint;
    in feature input;
    out feature output;
  }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-fact (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeFeatureMembership") (source "Model::Child") (target "Model::Child::owned") (outcome resolved))
  (type-derived-fact (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeFeature") (source "Model::Child") (target "Model::Child::owned") (outcome resolved))
  (type-derived-fact (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeEndFeature") (source "Model::Child") (target "Model::Child::endpoint") (outcome resolved))
  (type-derived-fact (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeDirectedFeature") (source "Model::Child") (target "Model::Child::input") (outcome resolved))
  (type-derived-fact (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeInheritedFeature") (source "Model::Child") (target "Model::Parent::inherited") (outcome resolved))
  (type-derived-fact (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeInput") (source "Model::Child") (target "Model::Child::input") (outcome resolved))
  (type-derived-fact (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeOutput") (source "Model::Child") (target "Model::Child::output") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_type_feature_collections.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:84edeb6e7046b42961dcc35c33a65f304b65ad428709290de7af4833b8df3677") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child"))) (kind kerml-type) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::endpoint"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)))
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::output"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::owned"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent"))) (kind kerml-type) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent::inherited"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child"))) (target (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::endpoint"))) (target (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::input"))) (target (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::output"))) (target (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::owned"))) (target (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent::inherited"))) (target (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child")))
      (supertype (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::endpoint")))
      (featured-by (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::input")))
      (featured-by (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::output")))
      (featured-by (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child::owned")))
      (featured-by (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent")))
      (subtype (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent::inherited")))
      (featured-by (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_type_feature_collections.md") (range (start 4 25) (end 4 31)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_type_feature_collections.md") (qualified-name "Model::Parent")))))
    )
  )
)
~~~
