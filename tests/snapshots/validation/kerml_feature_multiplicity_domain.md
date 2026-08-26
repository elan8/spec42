# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureMultiplicityDomain requires the featuringTypes of a Feature multiplicity to be the same as those of the Feature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureMultiplicityDomain
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureMultiplicityDomain
type=file
~~~
# SOURCE
~~~kerml
// Conforming: the multiplicity clause is attached to the feature declaration, so the
// multiplicity is featured by exactly the featuringTypes of the feature it bounds.
//
// The violating side has no textual counterpart: KerML concrete syntax offers no spelling that
// separates a feature's multiplicity from the feature's own featuring context, so a source
// document cannot give the multiplicity a different featuringType.
package Multiplicities {
    classifier Thing;
    classifier Holder {
        feature bounded[0..3] : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_multiplicity_domain.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_multiplicity_domain.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:dd0fd9ad199a5cdfbf65e453b91357aa43d48d66ea09b69087c7cd28dc4dcd9c") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder::bounded"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 3))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder::bounded"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder::bounded"))) (target (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder::bounded"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder::bounded"))) (target (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder::bounded")))
      (featured-by (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder::bounded")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (range (start 9 32) (end 9 37)) (probe (position 9 32))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Holder::bounded"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_multiplicity_domain.md") (qualified-name "Multiplicities::Thing")))))
    )
  )
)
~~~
