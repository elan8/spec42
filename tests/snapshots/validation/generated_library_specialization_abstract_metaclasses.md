# META
~~~ini
description=Generated specialization metaclasses that have no direct queryable authored concrete-syntax declaration remain visibly blocked
specification=OMG SysML 2.0 and KerML 1.0 (formal/26-03)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=by_construction
blocked_by=abstract-syntax-library-specialization-metaclasses
rule_id=kerml-1.0:8.3.4.8.10:checkLiteralExpressionSpecialization
rule_id=kerml-1.0:8.3.4.8.11:checkLiteralInfinitySpecialization
rule_id=kerml-1.0:8.3.4.8.12:checkLiteralIntegerSpecialization
rule_id=kerml-1.0:8.3.4.8.13:checkLiteralRationalSpecialization
rule_id=kerml-1.0:8.3.4.8.14:checkLiteralStringSpecialization
rule_id=kerml-1.0:8.3.4.8.15:checkMetadataAccessExpressionSpecialization
rule_id=kerml-1.0:8.3.4.8.16:checkNullExpressionSpecialization
rule_id=kerml-1.0:8.3.4.8.9:checkLiteralBooleanSpecialization
rule_id=kerml-1.0:8.3.4.5.2:checkBindingConnectorSpecialization
rule_id=kerml-1.0:8.3.4.9.2:checkFlowSpecialization
rule_id=kerml-1.0:8.3.4.9.6:checkSuccessionFlowSpecialization
rule_id=sysml-2.0:8.3.16.3:checkFlowUsageSpecialization
rule_id=sysml-2.0:8.3.16.4:checkSuccessionFlowUsageSpecialization
rule_id=sysml-2.0:8.3.17.6:checkControlNodeSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package GeneratedAbstractSpecializationMetaclasses {
    // These forms author expressions, but the named declaration is the enclosing expression
    // rather than the individual abstract metaclass instance.
    expr LiteralValues { 1 true "text" }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_abstract_metaclasses.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:fa544b5d78e51504f0567ad9ec24950e0d369d1eb654cf3e41a93b3716109a23") (contract-version "constructor-expression-specialization-v9") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_abstract_metaclasses.md") (qualified-name "GeneratedAbstractSpecializationMetaclasses"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_abstract_metaclasses.md") (qualified-name "GeneratedAbstractSpecializationMetaclasses::LiteralValues"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_abstract_metaclasses.md") (qualified-name "GeneratedAbstractSpecializationMetaclasses::LiteralValues"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/generated_library_specialization_abstract_metaclasses.md") (qualified-name "GeneratedAbstractSpecializationMetaclasses::LiteralValues"))) (state literal) (value (kind string) (value "text")))
    (evaluated (declaration (node (document "memory://snapshot/generated_library_specialization_abstract_metaclasses.md") (qualified-name "GeneratedAbstractSpecializationMetaclasses::LiteralValues"))) (state literal) (value (kind string) (value "text")))
    (evaluated (declaration (node (document "memory://snapshot/generated_library_specialization_abstract_metaclasses.md") (qualified-name "GeneratedAbstractSpecializationMetaclasses::LiteralValues"))) (state literal) (value (kind string) (value "text")))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_abstract_metaclasses.md") (qualified-name "GeneratedAbstractSpecializationMetaclasses::LiteralValues")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
