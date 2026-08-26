# META
~~~ini
description=KerML 8.3.3.3.8 validateRedefinitionFeaturingTypes requires a redefiningFeature to have at least one featuringType that is not also a featuringType of the redefinedFeature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.8 validateRedefinitionFeaturingTypes
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.8:validateRedefinitionFeaturingTypes
blocked_by=semantic-redefinition-featuring-type-compatibility
type=file
~~~
# SOURCE
~~~kerml
package Redefinitions {
    classifier Thing;
    classifier Base {
        feature original : Thing;
    }
    classifier Conforming specializes Base {
        // Conforming: the redefining feature is featured by Conforming, not by Base.
        feature original : Thing redefines Base::original;
    }
    classifier Invalid {
        feature first : Thing;

        // Invalid: both features are featured by Invalid alone.
        feature second : Thing redefines first;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_redefinition_featuring_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "redefinition_featuring_type_incompatible")
        (source "semantic")
        (range (start 13 8) (end 13 47))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_redefinition_featuring_types.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2509561083bc2998fc80a48f1d867d54d8870b72841043b0b722d00d8a353b66") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (redefinition (reference "Base::original")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (redefinition (reference "first")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (kind redefinition) (ordinal 0))
      (authored-target "Base::original")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (kind redefinition) (ordinal 0))
      (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base")))
      (subtype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base")))
      (type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming")))
      (supertype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming")))
      (type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original"))))
      (supertype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid")))
      (type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid")))
      (type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first"))))
      (supertype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_redefinition_featuring_types.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_featuring_types.md") (range (start 5 38) (end 5 42)) (probe (position 5 38))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_featuring_types.md") (range (start 7 27) (end 7 32)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_featuring_types.md") (range (start 7 43) (end 7 57)) (probe (position 7 43))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Conforming::original"))) (kind redefinition) (ordinal 0) (authored-target "Base::original")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Base::original")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_featuring_types.md") (range (start 10 24) (end 10 29)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_featuring_types.md") (range (start 13 25) (end 13 30)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_featuring_types.md") (range (start 13 41) (end 13 46)) (probe (position 13 41))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::second"))) (kind redefinition) (ordinal 0) (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_featuring_types.md") (qualified-name "Redefinitions::Invalid::first")))))
    )
  )
)
~~~
