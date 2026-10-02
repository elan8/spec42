# META
~~~ini
description=SysML 8.3.9.3 checkOccurrenceDefinitionMultiplicitySpecialization requires the multiplicity of an individual OccurrenceDefinition to specialize Base::zeroOrOne
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.9.3:checkOccurrenceDefinitionMultiplicitySpecialization
libraries=standard
~~~
# SOURCE
~~~sysml
package Model {
    // Each individual definition owns an empty Multiplicity that specializes Base::zeroOrOne.
    individual part def Earth;
    individual def Moon;
    // A non-individual definition owns no multiplicity and is not constrained.
    part def Planet;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (specialization-check
    (rule_id "sysml-2.0:8.3.9.3:checkOccurrenceDefinitionMultiplicitySpecialization")
    (outcome satisfied))
  (relationship
    (kind subsetting)
    (source (anonymous (owner "Model::Earth") (kind Multiplicity) (ordinal 0)))
    (target "Base::zeroOrOne")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind subsetting)
    (source (anonymous (owner "Model::Moon") (kind Multiplicity) (ordinal 0)))
    (target "Base::zeroOrOne")
    (provenance implied)
    (outcome resolved))
  (type-derived-fact
    (rule_id "kerml-1.0:8.3.3.1.10:deriveTypeMultiplicity")
    (source "Model::Planet")
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:04fdc1316711d149b2201eaa6201324e3c9c306fc735c8726fd42b976fc2f6da") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Earth"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Earth")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Moon"))) (kind individual-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind individual-definition) (name "Moon")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Planet"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Earth"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Earth")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Earth")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::zeroOrOne"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Moon"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind individual-definition) (name "Moon")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind individual-definition) (name "Moon")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::zeroOrOne"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Planet"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Earth")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Earth")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Moon")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind individual-definition) (name "Moon")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Planet")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
