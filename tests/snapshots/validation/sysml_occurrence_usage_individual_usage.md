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
libraries=standard
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
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8658aea9dbd5098eeb87cffae3e4a079cc419a1ce6105b212b332ea3dd29c278") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Plain")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Identified")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (path (named (kind package) (name "Occurrences")) (named (kind occurrence-def) (name "Identified")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (path (named (kind package) (name "Occurrences")) (named (kind occurrence-def) (name "Identified")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (path (named (kind package) (name "Occurrences")) (named (kind occurrence-def) (name "Identified")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::zeroOrOne"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Identified")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Holder::good")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (path (named (kind package) (name "Occurrences")) (named (kind occurrence-def) (name "Identified")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Natural")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::zeroOrOne")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Natural")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_usage.md") (qualified-name "Occurrences::Plain")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
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
