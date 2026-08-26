# META
~~~ini
description=KerML 8.3.3.3.8 validateRedefinitionDirectionConformance requires a redefiningFeature to keep the direction of the redefinedFeature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.8 validateRedefinitionDirectionConformance
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.8:validateRedefinitionDirectionConformance
type=file
~~~
# SOURCE
~~~kerml
package Redefinitions {
    classifier Thing;
    behavior Base {
        in feature input : Thing;
    }
    behavior Conforming specializes Base {
        // Conforming: the redefining feature keeps the in direction.
        in feature input : Thing;
    }
    behavior Invalid specializes Base {
        // Invalid: the redefining feature reverses the direction.
        out feature input : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_redefinition_direction_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "redefinition_direction_mismatch")
        (source "semantic")
        (range (start 11 8) (end 11 34))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_redefinition_direction_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "redefinition_direction_mismatch")
        (source "semantic")
        (range (start 11 8) (end 11 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8510ceccbf57bac9beb20013acd138d9198d3a524e9f86810b6bc720a9401dab") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction out)))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input"))) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base")))
      (subtype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base")))
      (type (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming")))
      (supertype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming")))
      (type (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))))
      (supertype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid")))
      (supertype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid")))
      (type (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))))
      (supertype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (range (start 5 36) (end 5 40)) (probe (position 5 36))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (range (start 7 27) (end 7 32)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Conforming::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (range (start 9 33) (end 9 37)) (probe (position 9 33))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (range (start 11 28) (end 11 33)) (probe (position 11 28))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Invalid::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_direction_conformance.md") (qualified-name "Redefinitions::Thing")))))
    )
  )
)
~~~
