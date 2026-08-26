# META
~~~ini
description=SysML checkOccurrenceDefinitionMultiplicitySpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.9.3:checkOccurrenceDefinitionMultiplicitySpecialization
blocked_by=lowering-gap-specialization-individual-multiplicity-library-anchor
~~~
# SOURCE
~~~sysml
package Model { part def Parent; part def Child :> Parent; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1f306929b65664fd34e1e264ad7ed9da37ea374b6c1a036feef9e48d715f01cc") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Parent"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Parent")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Child"))) (target (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Child")))
      (supertype (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Parent")))
      (subtype (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Child")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (range (start 0 51) (end 0 57)) (probe (position 0 51))
    (reference (id (source (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_occurrence_definition_multiplicity_specialization.md") (qualified-name "Model::Parent")))))
    )
  )
)
~~~
