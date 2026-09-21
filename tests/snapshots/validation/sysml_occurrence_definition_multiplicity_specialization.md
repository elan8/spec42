# META
~~~ini
description=SysML checkOccurrenceDefinitionMultiplicitySpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.9.3:checkOccurrenceDefinitionMultiplicitySpecialization
libraries=standard
~~~
# SOURCE
~~~sysml
package Model { individual occurrence def IndividualLife; occurrence def Ordinary; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "sysml-2.0:8.3.9.3:checkOccurrenceDefinitionMultiplicitySpecialization") (outcome satisfied)))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:c6d25865490f615bf39eec4bd643b2e60fd91c7ec23fcc7adf17638e1a5d5b99") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::IndividualLife"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "IndividualLife")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Ordinary"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::IndividualLife"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "IndividualLife")) (anonymous (kind kerml-multiplicity) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::IndividualLife")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Life")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (path (named (kind package) (name "Model")) (named (kind occurrence-def) (name "IndividualLife")) (anonymous (kind kerml-multiplicity) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Natural")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
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
)
~~~
