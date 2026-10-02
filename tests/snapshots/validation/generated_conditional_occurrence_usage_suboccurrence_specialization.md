# META
~~~ini
description=A composite occurrence usage owned by a class specializes Occurrences::Occurrence::suboccurrences
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.9.4:checkOccurrenceUsageSuboccurrenceSpecialization
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
  (relationship (kind subsetting) (source "OccurrenceUsageSuboccurrenceSpecialization::Container::child") (target "Occurrences::Occurrence::suboccurrences") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5edb2e063f93a62dc89b1e9b5ce4df850451b19287b929c98751f05571f44374") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container::child"))) (kind occurrence) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container::child"))) (target (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container::child"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container::child"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container::child")))
      (featured-by (node (document "memory://snapshot/generated_conditional_occurrence_usage_suboccurrence_specialization.md") (qualified-name "OccurrenceUsageSuboccurrenceSpecialization::Container")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
