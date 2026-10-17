# META
~~~ini
description=KerML deriveFeatureChainingFeature and deriveFeatureOwnedFeatureInverting project canonical relationships
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 deriveFeatureChainingFeature
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureChainingFeature
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureInverting
libraries=none
type=file
~~~
# SOURCE
~~~kerml
package Model {
    classifier Vehicle {
        feature parent;
        feature child inverse of parent;
        feature base;
        feature path chains base;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureChainingFeature")
    (source "Model::Vehicle::path")
    (kind feature_chaining)
    (target "Model::Vehicle::base")
    (provenance authored)
    (outcome resolved))
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureInverting")
    (source "Model::Vehicle::child")
    (kind feature_inverting)
    (target "Model::Vehicle::parent")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chaining_and_inverting.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_chaining_single_operand")
        (source "semantic")
        (range (start 5 28) (end 5 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:af027364c35f512fea590b1487b33d47a9758e7889c0f3b5d124ce0b06c9d58d"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::child"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureInverting (reference "parent")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::parent"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::path"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureChaining (reference "base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::child"))) (kind featureInverting) (ordinal 0))
      (authored-target "parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::parent")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::path"))) (kind featureChaining) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::base")))))
  )
  (relationships
    (relationship (kind featureInverting) (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::child"))) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::child"))) (kind featureInverting) (ordinal 0)))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::path"))) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::path"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::base"))) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::child"))) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::parent"))) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::path"))) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::base")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::child")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::parent")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::path")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (range (start 3 33) (end 3 39)) (probe (position 3 33))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::child"))) (kind featureInverting) (ordinal 0) (authored-target "parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::parent")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (range (start 5 28) (end 5 32)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::path"))) (kind featureChaining) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_and_inverting.md") (qualified-name "Model::Vehicle::base")))))
    )
  )
)
~~~
