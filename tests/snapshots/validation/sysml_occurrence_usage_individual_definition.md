# META
~~~ini
description=SysML 8.3.9.4 validateOccurrenceUsageIndividualDefinition allows an OccurrenceUsage at most one occurrenceDefinition with isIndividual = true
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.9.4 validateOccurrenceUsageIndividualDefinition
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.9.4:validateOccurrenceUsageIndividualDefinition
blocked_by=semantic-occurrence-multiple-individual-definitions
type=file
~~~
# SOURCE
~~~sysml
package Occurrences {
    individual occurrence def First;
    individual occurrence def Second;
    occurrence def Plain;
    part def Holder {
        // Conforming: at most one individual occurrence definition among the types.
        occurrence good : First, Plain;

        // Invalid: two individual occurrence definitions among the types.
        occurrence bad : First, Second;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "occurrence_multiple_individual_definitions")
        (source "semantic")
        (range (start 9 8) (end 9 39))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:246477e3dcd785ae31eb8dd0150dfdad40b20d8340928494e2f75842a3e05a44") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "First")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "First")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (range (start 9 25) (end 9 30)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))))
    )
  )
  (query (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (range (start 6 26) (end 6 31)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))))
    )
  )
)
~~~
