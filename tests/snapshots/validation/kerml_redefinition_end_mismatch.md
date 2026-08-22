# META
~~~ini
description=KerML 8.3.3.3.8 validateRedefinitionEndConformance requires a Feature redefining an end Feature to also be an end Feature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.8 validateRedefinitionEndConformance
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.8:validateRedefinitionEndConformance
type=file
~~~
# SOURCE
~~~kerml
package Ends {
    classifier Thing;
    assoc Base {
        end feature endpoint : Thing;
    }

    // Conforming: the redefining feature remains an end feature.
    assoc Conforming specializes Base {
        end feature endpoint : Thing;
    }

    // Invalid: the implied redefinition drops the end modifier.
    assoc Invalid specializes Base {
        feature endpoint : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_redefinition_end_mismatch.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "redefinition_end_mismatch")
        (source "semantic")
        (range (start 13 8) (end 13 33))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_redefinition_end_mismatch.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "redefinition_end_mismatch")
        (source "semantic")
        (range (start 13 8) (end 13 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:efb20e355a1a9befefc180f9df9ad5b11e6f04d73945790ff8201feb293e7b7a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint"))) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base")))
      (subtype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base")))
      (type (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming")))
      (supertype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming")))
      (type (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))))
      (supertype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid")))
      (supertype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint")))
      (featured-by (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid")))
      (type (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))))
      (supertype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (range (start 3 31) (end 3 36)) (probe (position 3 31))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base::endpoint"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (range (start 7 33) (end 7 37)) (probe (position 7 33))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (range (start 8 31) (end 8 36)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Conforming::endpoint"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (range (start 12 30) (end 12 34)) (probe (position 12 30))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (range (start 13 27) (end 13 32)) (probe (position 13 27))
    (reference (id (source (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Invalid::endpoint"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_redefinition_end_mismatch.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
