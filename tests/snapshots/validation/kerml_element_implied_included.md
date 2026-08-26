# META
~~~ini
description=KerML 8.3.2.1.2 validateElementIsImpliedIncluded requires an Element owning any implied Relationship to have isImpliedIncluded = true
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.2.1.2 validateElementIsImpliedIncluded
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.2.1.2:validateElementIsImpliedIncluded
type=file
~~~
# SOURCE
~~~kerml
// Conforming: Special::part carries an implied redefinition of General::part, and the
// publication records that implied relationship on the redefining feature itself.
//
// The violating side of this constraint has no textual counterpart: isImplied and
// isImpliedIncluded are abstract-syntax bookkeeping attributes with no concrete syntax, so a
// KerML source document cannot author an Element that owns an implied Relationship while
// leaving isImpliedIncluded = false. The rule is therefore observable only as the accepted
// side pinned here, plus the (provenance implied) fact in SMG.
package Implied {
    classifier Thing;
    classifier General {
        feature part : Thing;
    }
    classifier Special specializes General {
        feature part : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_element_implied_included.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_element_implied_included.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ab30b0f073cfbeeb946e7e406c3b97dead32ee47a9913d122e47fe5aa25b509a") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "General")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special"))) (kind specialization) (ordinal 0))
      (authored-target "General")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part"))) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special"))) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part"))) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part"))) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part"))) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part"))) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General")))
      (subtype (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part")))
      (featured-by (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General")))
      (type (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special")))
      (supertype (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part")))
      (featured-by (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special")))
      (type (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part"))))
      (supertype (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")))
      (subtype (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_element_implied_included.md") (range (start 11 23) (end 11 28)) (probe (position 11 23))
    (reference (id (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General::part"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_implied_included.md") (range (start 13 35) (end 13 42)) (probe (position 13 35))
    (reference (id (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special"))) (kind specialization) (ordinal 0) (authored-target "General")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::General")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_implied_included.md") (range (start 14 23) (end 14 28)) (probe (position 14 23))
    (reference (id (source (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Special::part"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_implied_included.md") (qualified-name "Implied::Thing")))))
    )
  )
)
~~~
