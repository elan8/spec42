# META
~~~ini
description=A classifier projects owned subclassifications, and a feature projects its first reference and cross subsetting
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.2.2:deriveClassifierOwnedSubclassification
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedReferenceSubsetting
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureOwnedCrossSubsetting
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
    classifier Base;
    classifier Derived specializes Base;
    classifier Thing {
        feature inner;
    }
    assoc Link {
        end feature source : Thing;
        end feature crossed : Thing crosses source.inner;
    }
    classifier Vehicle {
        feature target;
        feature proxy references target;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (type-derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.2.2:deriveClassifierOwnedSubclassification")
    (source "Model::Derived")
    (kind specialization)
    (target "Model::Base")
    (provenance authored)
    (outcome resolved))
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedReferenceSubsetting")
    (source "Model::Vehicle::proxy")
    (kind reference_subsetting)
    (target "Model::Vehicle::target")
    (provenance authored)
    (outcome resolved))
  (derived-relationship-collection
    (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedCrossSubsetting")
    (source "Model::Link::crossed")
    (kind cross_subsetting)
    (target "Model::Thing::inner")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_subclassification_and_subsetting.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "cross_feature_type_mismatch")
        (source "semantic")
        (range (start 8 44) (end 8 56))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:52bb3ea4c3508d6f619e85289d7ad4d6f4e33b7a28556c3035fcca39c0e6bb88"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Derived"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "source::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::proxy"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "target")))))
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::proxy"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::target")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Derived"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::source"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::proxy"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::proxy"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::source"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing::inner"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::proxy"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::target"))) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Derived")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Derived")))
      (supertype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed")))
      (featured-by (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link")))
      (type (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing::inner")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::source")))
      (featured-by (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link")))
      (type (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::proxy")))
      (featured-by (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle")))
      (supertype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::target")))
      (featured-by (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle")))
      (subtype (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::proxy")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (range (start 2 35) (end 2 39)) (probe (position 2 35))
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Derived"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (range (start 8 30) (end 8 35)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (range (start 8 44) (end 8 56)) (probe (position 8 44))
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::crossed"))) (kind crossSubsetting) (ordinal 0) (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (range (start 7 29) (end 7 34)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (range (start 12 33) (end 12 39)) (probe (position 12 33))
    (reference (id (source (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::proxy"))) (kind referenceSubsetting) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subclassification_and_subsetting.md") (qualified-name "Model::Vehicle::target")))))
    )
  )
)
~~~
