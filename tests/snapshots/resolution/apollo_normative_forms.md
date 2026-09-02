# META
~~~ini
description=The five Apollo 11 normative textual forms of issue #100: forms 1, 3, and 5 (keyword-less feature usage in a use-case-family body, multiplicity before typing on a nested action usage, and a def-less abstract connection usage) lower without recovery or unsupported-form diagnostics; forms 2 and 4 stay blocked on parser gaps 83 and 84
type=file
~~~
# SOURCE
~~~sysml
package ApolloNormativeForms {
    part def SaturnV;
    part def MissionSystem {
        part launchVehicle : SaturnV;
    }
    part apollo11MissionSystem : MissionSystem;

    // Form 1: keyword-less feature usage with an explicit typing and value in an
    // `analysis` (use-case-family) body -- `recovered_use_case_body_element` before the bump.
    analysis Apollo11MissionDeltaVBudgetAnalysis {
        launchVehicle : SaturnV = apollo11MissionSystem.launchVehicle;
    }

    // Form 3: a nested `action` usage whose multiplicity precedes the typing and `:>>`
    // redefinition -- `recovered_action_body_element` before the bump.
    action def Function {
        action subactions[*] : Function;
    }
    abstract action def CompositeFunction :> Function {
        action subfunctions[*] : Function :>> subactions;
    }

    // Form 5: a def-less `abstract connection` usage of the same multiplicity-first shape --
    // both `unsupported_grammar_form` and `unsupported_parser_construct` before the bump.
    connection def CapabilityToGoalDerivation;
    part def CapabilitySet {
        abstract connection capabilityToGoals[*] : CapabilityToGoalDerivation;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/apollo_normative_forms.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/apollo_normative_forms.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:339e6303c33ca86e19f8a9360ec0a83e75ccb91dc49a4e8644dcb272c87875cb"))
  (declarations
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis"))) (kind analysis) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis::launchVehicle"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SaturnV")))))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "apollo11MissionSystem::launchVehicle")))))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (kind kerml-feature) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet::capabilityToGoals"))) (kind connection) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CapabilityToGoalDerivation")))))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilityToGoalDerivation"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Function")))))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Function")) (redefinition (reference "subactions")))))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Function")))))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SaturnV")))))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::apollo11MissionSystem"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissionSystem")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis::launchVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "SaturnV")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")))))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "apollo11MissionSystem::launchVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle")))))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet::capabilityToGoals"))) (kind featureTyping) (ordinal 0))
      (authored-target "CapabilityToGoalDerivation")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilityToGoalDerivation")))))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction"))) (kind specialization) (ordinal 0))
      (authored-target "Function")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")))))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (kind featureTyping) (ordinal 0))
      (authored-target "Function")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")))))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (kind redefinition) (ordinal 0))
      (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions")))))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions"))) (kind featureTyping) (ordinal 0))
      (authored-target "Function")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")))))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "SaturnV")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")))))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::apollo11MissionSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissionSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis::launchVehicle"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis::launchVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet::capabilityToGoals"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilityToGoalDerivation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet::capabilityToGoals"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::apollo11MissionSystem"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::apollo11MissionSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis::launchVehicle"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet::capabilityToGoals"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle"))) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis::launchVehicle")))
      (featured-by (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis")))
      (type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")) (provenance authored))
      (effective-type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")) (source direct))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")) (source inherited) (from (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle"))))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle")) (scopes any feature))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet::capabilityToGoals")))
      (featured-by (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet")))
      (type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilityToGoalDerivation")) (provenance authored))
      (effective-type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilityToGoalDerivation")) (source direct))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilityToGoalDerivation")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilityToGoalDerivation")))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet::capabilityToGoals")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction")))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions")))
      (featured-by (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction")))
      (type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")) (provenance authored))
      (effective-type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")) (source direct))
      (effective-type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")) (source inherited) (from (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions"))))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")) (scopes any))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions")) (scopes any))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions")))
      (featured-by (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")))
      (type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")) (provenance authored))
      (effective-type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")) (source direct))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")) (scopes any))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem")))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::apollo11MissionSystem")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle")))
      (featured-by (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem")))
      (type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")) (provenance authored))
      (effective-type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")) (source direct))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")) (scopes any))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis::launchVehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::apollo11MissionSystem")))
      (type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem")) (provenance authored))
      (effective-type (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem")) (source direct))
      (supertype (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/apollo_normative_forms.md") (range (start 10 24) (end 10 31)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Apollo11MissionDeltaVBudgetAnalysis::launchVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "SaturnV")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")))))
    )
  )
  (query (document "memory://snapshot/apollo_normative_forms.md") (range (start 10 34) (end 10 69)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (path (named (kind package) (name "ApolloNormativeForms")) (named (kind analysis) (name "Apollo11MissionDeltaVBudgetAnalysis")) (named (kind default-reference) (name "launchVehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "apollo11MissionSystem::launchVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle")))))
    )
  )
  (query (document "memory://snapshot/apollo_normative_forms.md") (range (start 26 51) (end 26 77)) (probe (position 26 51))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilitySet::capabilityToGoals"))) (kind featureTyping) (ordinal 0) (authored-target "CapabilityToGoalDerivation")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CapabilityToGoalDerivation")))))
    )
  )
  (query (document "memory://snapshot/apollo_normative_forms.md") (range (start 18 45) (end 18 53)) (probe (position 18 45))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction"))) (kind specialization) (ordinal 0) (authored-target "Function")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")))))
    )
  )
  (query (document "memory://snapshot/apollo_normative_forms.md") (range (start 19 33) (end 19 41)) (probe (position 19 33))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (kind featureTyping) (ordinal 0) (authored-target "Function")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")))))
    )
  )
  (query (document "memory://snapshot/apollo_normative_forms.md") (range (start 19 46) (end 19 56)) (probe (position 19 46))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::CompositeFunction::subfunctions"))) (kind redefinition) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions")))))
    )
  )
  (query (document "memory://snapshot/apollo_normative_forms.md") (range (start 16 31) (end 16 39)) (probe (position 16 31))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function::subactions"))) (kind featureTyping) (ordinal 0) (authored-target "Function")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::Function")))))
    )
  )
  (query (document "memory://snapshot/apollo_normative_forms.md") (range (start 3 29) (end 3 36)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem::launchVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "SaturnV")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::SaturnV")))))
    )
  )
  (query (document "memory://snapshot/apollo_normative_forms.md") (range (start 5 33) (end 5 46)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::apollo11MissionSystem"))) (kind featureTyping) (ordinal 0) (authored-target "MissionSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/apollo_normative_forms.md") (qualified-name "ApolloNormativeForms::MissionSystem")))))
    )
  )
)
~~~
