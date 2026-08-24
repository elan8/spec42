# META
~~~ini
description=SysML 8.3.9.2 validateEventOccurrenceUsageReference requires the featureTarget of the referencedFeature of an EventOccurrenceUsage ownedReferenceSubsetting to be an OccurrenceUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.9.2 validateEventOccurrenceUsageReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.9.2:validateEventOccurrenceUsageReference
blocked_by=lowering-event-occurrence-reference
type=file
~~~
# SOURCE
~~~sysml
package Events {
    action def Performing {
        occurrence milestone;
        ref attribute reading;

        // Conforming: the referenced feature is an occurrence usage.
        event occurrence good references milestone;

        // Invalid: the referenced feature is an attribute usage, not an occurrence usage.
        event occurrence bad references reading;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_event_occurrence_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "event_occurrence_reference_not_occurrence")
        (source "semantic")
        (range (start 9 8) (end 9 48))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_event_occurrence_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_subset_redefine_kind")
        (source "semantic")
        (range (start 9 40) (end 9 47))
        (related-information
          (related
            (uri "memory://snapshot/sysml_event_occurrence_usage_reference.md")
            (range (start 3 8) (end 3 30))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cbb3fe2e13d9390ff6c99d2650d5b330a39492d202b1cbcc6f45740de83b5768") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::bad"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::good"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "milestone")))))
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::milestone"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::reading"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers reference)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::bad"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::reading")))))
    (reference (id (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::good"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "milestone")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::milestone")))))
  )
  (relationships
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::bad"))) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::bad"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::good"))) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::milestone"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::good"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::bad"))) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::good"))) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::milestone"))) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::reading"))) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::bad")))
      (featured-by (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::good")))
      (featured-by (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::milestone")))
      (featured-by (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::reading")))
      (featured-by (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (range (start 9 40) (end 9 47)) (probe (position 9 40))
    (reference (id (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::bad"))) (kind referenceSubsetting) (ordinal 0) (authored-target "reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::reading")))))
    )
  )
  (query (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (range (start 6 41) (end 6 50)) (probe (position 6 41))
    (reference (id (source (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::good"))) (kind referenceSubsetting) (ordinal 0) (authored-target "milestone")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_reference.md") (qualified-name "Events::Performing::milestone")))))
    )
  )
)
~~~
