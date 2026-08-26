# META
~~~ini
description=Generated conditional library-specialization rules publish implied anchors only when their owned predicate facts hold
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.9.3:checkOccurrenceDefinitionIndividualSpecialization
rule_id=sysml-2.0:8.3.9.4:checkOccurrenceUsageSnapshotSpecialization
rule_id=sysml-2.0:8.3.9.4:checkOccurrenceUsageTimeSliceSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package ConditionalSpecializations {
    individual occurrence def Individual;
    individual occurrence def Explicit specializes Occurrences::Life;
    occurrence def Ordinary;
    snapshot occurrence Snapshot;
    timeslice occurrence Timeslice;
    occurrence Plain;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "ConditionalSpecializations::Individual") (target "Occurrences::Life") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "ConditionalSpecializations::Snapshot") (target "Occurrences::Occurrence::snapshots") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "ConditionalSpecializations::Timeslice") (target "Occurrences::Occurrence::timeSlices") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "ConditionalSpecializations::Explicit") (provenance implied) (outcome absent))
  (relationship (kind specialization) (source "ConditionalSpecializations::Ordinary") (provenance implied) (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_library_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:16f9fee4a0d6534062dafbdcad03d587adc1314197f40b277b427257413a6b0d") (contract-version "constructor-expression-specialization-v9") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Explicit"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Occurrences::Life")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Individual"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Ordinary"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Plain"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Snapshot"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Timeslice"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Explicit"))) (kind specialization) (ordinal 0))
      (authored-target "Occurrences::Life")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Explicit"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Explicit"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Individual"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Plain"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Snapshot"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Snapshot"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Timeslice"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Timeslice"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Explicit")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Individual")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Plain")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Snapshot")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Timeslice")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_library_specializations.md") (range (start 2 51) (end 2 68)) (probe (position 2 51))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_library_specializations.md") (qualified-name "ConditionalSpecializations::Explicit"))) (kind specialization) (ordinal 0) (authored-target "Occurrences::Life")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")))))
    )
  )
)
~~~
