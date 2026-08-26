# META
~~~ini
description=SysML 8.3.9.2 validateEventOccurrenceUsageIsReference requires an EventOccurrenceUsage to be referential
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.9.2 validateEventOccurrenceUsageIsReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.9.2:validateEventOccurrenceUsageIsReference
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the event occurrence keyword pair produces an EventOccurrenceUsage and makes it
// referential at the same time.
//
// The violating side has no textual counterpart: SysML concrete syntax has no spelling that
// produces a composite EventOccurrenceUsage, so the rule is observable only as the accepted side
// pinned here.
package Events {
    action def Performing {
        event occurrence started;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_event_occurrence_usage_is_reference.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_event_occurrence_usage_is_reference.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:54deefe830e92e740caf3ddfb58b5c09f4aa61da520f105132bcc5ec3df10da0") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_is_reference.md") (qualified-name "Events"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_is_reference.md") (qualified-name "Events::Performing"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_is_reference.md") (qualified-name "Events::Performing::started"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_event_occurrence_usage_is_reference.md") (qualified-name "Events::Performing::started"))) (target (node (document "memory://snapshot/sysml_event_occurrence_usage_is_reference.md") (qualified-name "Events::Performing"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_event_occurrence_usage_is_reference.md") (qualified-name "Events::Performing::started")))
      (featured-by (node (document "memory://snapshot/sysml_event_occurrence_usage_is_reference.md") (qualified-name "Events::Performing")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
