# META
~~~ini
description=SysML 8.3.9.4 validateOccurrenceUsageIsPortion requires an OccurrenceUsage with a non-null portionKind to have isPortion = true
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.9.4 validateOccurrenceUsageIsPortion
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.9.4:validateOccurrenceUsageIsPortion
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the snapshot and timeslice keywords set the portionKind and isPortion of an
// occurrence usage together.
//
// The violating side has no textual counterpart: SysML concrete syntax derives both facts from
// the same keyword, so a source document cannot author a portion kind without isPortion.
package Occurrences {
    occurrence def Event;
    part def Holder {
        snapshot instant : Event;
        timeslice interval : Event;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_occurrence_usage_is_portion.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_occurrence_usage_is_portion.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f68f32455833d69fb8356cf26acf73f660bbd99f90f733867ca77e13065c39ea") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::instant"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Event")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::interval"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Event")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::instant"))) (kind featureTyping) (ordinal 0))
      (authored-target "Event")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")))))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::interval"))) (kind featureTyping) (ordinal 0))
      (authored-target "Event")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::instant"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::instant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::interval"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::interval"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::instant"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::interval"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::instant")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::interval")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::instant")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::interval")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (range (start 8 27) (end 8 32)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::instant"))) (kind featureTyping) (ordinal 0) (authored-target "Event")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")))))
    )
  )
  (query (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (range (start 9 29) (end 9 34)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Holder::interval"))) (kind featureTyping) (ordinal 0) (authored-target "Event")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_is_portion.md") (qualified-name "Occurrences::Event")))))
    )
  )
)
~~~
