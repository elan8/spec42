# META
~~~ini
description=OccurrenceUsage suboccurrence specialization remains explicit until canonical Class-category owner facts are published
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.9.4:checkOccurrenceUsageSuboccurrenceSpecialization
blocked_by=lowering-gap-occurrence-usage-suboccurrence-category-facts
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package OccurrenceUsageSuboccurrenceSpecialization {
    part def Container {
        occurrence child;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "OccurrenceUsageSuboccurrenceSpecialization::Container::child") (target "Occurrences::Occurrence::suboccurrences") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:c06806b4147dbcec1b32788f8366e60fcdcafef4679a6549879319e17bf9fc8d") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container::child"))) (kind occurrence) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container::child"))) (target (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container::child"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container::child")))
      (featured-by (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_lowering_gap.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
