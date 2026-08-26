# META
~~~ini
description=SysML 8.3.9.4 validateOccurrenceUsagePortionKind requires the owningType of an OccurrenceUsage with a non-null portionKind to be an OccurrenceDefinition or an OccurrenceUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.9.4 validateOccurrenceUsagePortionKind
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.9.4:validateOccurrenceUsagePortionKind
blocked_by=semantic-portion-owner-not-occurrence
type=file
~~~
# SOURCE
~~~sysml
package Occurrences {
    occurrence def Event;

    // Conforming: the portion is owned by an occurrence definition.
    occurrence def Good {
        snapshot instant : Event;
    }

    // Invalid: the portion is owned by an attribute definition.
    attribute def Bad {
        snapshot instant : Event;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "portion_owner_not_occurrence")
        (source "semantic")
        (range (start 5 8) (end 5 33))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:dab930329f4aa3537b4d2a7620f684fb242d1a0d65601d609fb9c07a070ded64") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad::instant"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Event")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good::instant"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Event")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad::instant"))) (kind featureTyping) (ordinal 0))
      (authored-target "Event")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")))))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good::instant"))) (kind featureTyping) (ordinal 0))
      (authored-target "Event")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad::instant"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad::instant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good::instant"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good::instant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad::instant"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good::instant"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad::instant")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad::instant")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good::instant")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good::instant")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (range (start 10 27) (end 10 32)) (probe (position 10 27))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Bad::instant"))) (kind featureTyping) (ordinal 0) (authored-target "Event")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")))))
    )
  )
  (query (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (range (start 5 27) (end 5 32)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Good::instant"))) (kind featureTyping) (ordinal 0) (authored-target "Event")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_portion_kind.md") (qualified-name "Occurrences::Event")))))
    )
  )
)
~~~
