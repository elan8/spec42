# META
~~~ini
description=SysML 8.3.9.4 validateOccurrenceUsageIndividualUsage requires an OccurrenceUsage with isIndividual = true to have an individualDefinition
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.9.4 validateOccurrenceUsageIndividualUsage
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.9.4:validateOccurrenceUsageIndividualUsage
blocked_by=semantic-individual-usage-without-individual-definition
type=file
~~~
# SOURCE
~~~sysml
package Occurrences {
    individual occurrence def Identified;
    occurrence def Plain;
    part def Holder {
        // Conforming: the individual usage is typed by an individual occurrence definition.
        individual occurrence good : Identified;

        // Invalid: the individual usage has no individual occurrence definition.
        individual occurrence bad : Plain;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "individual_usage_without_individual_definition")
        (source "semantic")
        (range (start 8 8) (end 8 42))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c16fe6bdb1ad6e97fd19940590352e24d07b4669e5d7cf33c0faa11d24f293ba") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Plain")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Identified")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")))))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Identified")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (range (start 8 36) (end 8 41)) (probe (position 8 36))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")))))
    )
  )
  (query (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (range (start 5 37) (end 5 47)) (probe (position 5 37))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Identified")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")))))
    )
  )
)
~~~
