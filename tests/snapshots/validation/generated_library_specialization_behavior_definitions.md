# META
~~~ini
description=Generated library-specialization checks publish implied anchors for behavior and evaluation definitions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.3:checkActionDefinitionSpecialization
rule_id=sysml-2.0:8.3.18.5:checkStateDefinitionSpecialization
rule_id=sysml-2.0:8.3.19.2:checkCalculationDefinitionSpecialization
rule_id=sysml-2.0:8.3.20.3:checkConstraintDefinitionSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package GeneratedBehaviorDefinitions {
    action def ActionDefinition;
    state def StateDefinition;
    calc def CalculationDefinition;
    constraint def ConstraintDefinition;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "GeneratedBehaviorDefinitions::ActionDefinition") (target "Actions::Action") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedBehaviorDefinitions::StateDefinition") (target "States::StateAction") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedBehaviorDefinitions::CalculationDefinition") (target "Calculations::Calculation") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedBehaviorDefinitions::ConstraintDefinition") (target "Constraints::ConstraintCheck") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_behavior_definitions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:e589a110004046c0da345f3e3c038819eb2dcc8953b653ceec53dce3a332d235") (contract-version "semantic-metadata-projection-v6") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::ActionDefinition"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::CalculationDefinition"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::ConstraintDefinition"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::StateDefinition"))) (kind state-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::ActionDefinition"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::CalculationDefinition"))) (target (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::ConstraintDefinition"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::StateDefinition"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::ActionDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::CalculationDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::ConstraintDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_behavior_definitions.md") (qualified-name "GeneratedBehaviorDefinitions::StateDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
