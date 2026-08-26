# META
~~~ini
description=KerML 8.3.3.3.10 validateSubsettingFeaturingTypes requires the subsettedFeature of a Subsetting to be accessible by the subsettingFeature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.10 validateSubsettingFeaturingTypes
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.10:validateSubsettingFeaturingTypes
blocked_by=semantic-subsetting-target-not-accessible
type=file
~~~
# SOURCE
~~~kerml
package Subsettings {
    classifier Thing;
    classifier Base {
        feature inherited : Thing;
    }
    classifier Conforming specializes Base {
        // Conforming: the subsetted feature is accessible through the specialization.
        feature narrowed : Thing subsets inherited;
    }
    classifier Unrelated {
        // Invalid: Base::inherited is not accessible from an unrelated type.
        feature foreign : Thing subsets Base::inherited;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_subsetting_featuring_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "subsetting_target_not_accessible")
        (source "semantic")
        (range (start 11 40) (end 11 55))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_subsetting_featuring_types.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:fcbc5eeb8f1cd26f302306170a4ec3f30c93f15f8147a20813227eec226e5a89") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "inherited")))))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "Base::inherited")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (kind subsetting) (ordinal 0))
      (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (kind subsetting) (ordinal 0))
      (authored-target "Base::inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming"))) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base")))
      (subtype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited")))
      (featured-by (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base")))
      (type (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming")))
      (supertype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed")))
      (featured-by (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming")))
      (type (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))))
      (supertype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign")))
      (featured-by (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated")))
      (type (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))))
      (supertype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_subsetting_featuring_types.md") (range (start 3 28) (end 3 33)) (probe (position 3 28))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_featuring_types.md") (range (start 5 38) (end 5 42)) (probe (position 5 38))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_featuring_types.md") (range (start 7 27) (end 7 32)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_featuring_types.md") (range (start 7 41) (end 7 50)) (probe (position 7 41))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Conforming::narrowed"))) (kind subsetting) (ordinal 0) (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_featuring_types.md") (range (start 11 26) (end 11 31)) (probe (position 11 26))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_featuring_types.md") (range (start 11 40) (end 11 55)) (probe (position 11 40))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Unrelated::foreign"))) (kind subsetting) (ordinal 0) (authored-target "Base::inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_featuring_types.md") (qualified-name "Subsettings::Base::inherited")))))
    )
  )
)
~~~
