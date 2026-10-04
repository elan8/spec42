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
libraries=standard
type=file
~~~
# SOURCE
~~~sysml
package Occurrences {
    individual part def First;
    individual part def Second;
    part def Plain;
    part def Holder {
        // Conforming: at most one individual occurrence definition among the types.
        part good : First, Plain;

        // Invalid: two individual occurrence definitions among the types.
        part bad : First, Second;
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
        (range (start 9 8) (end 9 33))
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
      (diagnostic
        (severity warning)
        (code "occurrence_multiple_individual_definitions")
        (source "semantic")
        (range (start 9 8) (end 9 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:575be56ef7308ad3eddcbb875b7616f59f7555591add3b621906978f31802548") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual) (individual-multiplicity (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "First")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "First")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (origin individual-multiplicity)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "First")) (featureTyping (reference "Second")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "First")) (featureTyping (reference "Plain")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual) (individual-multiplicity (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "Second")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "Second")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (origin individual-multiplicity)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 1))
      (authored-target "Second")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second")))))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 1))
      (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "First")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "First")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::zeroOrOne"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "Second")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "Second")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::zeroOrOne"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "First")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder")))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (provenance authored))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder")))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (provenance authored))
      (type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain")))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (path (named (kind package) (name "Occurrences")) (named (kind part-def) (name "Second")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))
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
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (range (start 9 19) (end 9 24)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))))
    )
  )
  (query (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (range (start 9 26) (end 9 32)) (probe (position 9 26))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::bad"))) (kind featureTyping) (ordinal 1) (authored-target "Second")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Second")))))
    )
  )
  (query (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (range (start 6 20) (end 6 25)) (probe (position 6 20))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::First")))))
    )
  )
  (query (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (range (start 6 27) (end 6 32)) (probe (position 6 27))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Holder::good"))) (kind featureTyping) (ordinal 1) (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_usage_individual_definition.md") (qualified-name "Occurrences::Plain")))))
    )
  )
)
~~~
