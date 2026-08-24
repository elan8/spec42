# META
~~~ini
description=RequirementDefinition exact derived properties project canonical membership roles and documentation bodies
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.21.8:deriveRequirementDefinitionActorParameter
rule_id=sysml-2.0:8.3.21.8:deriveRequirementDefinitionSubjectParameter
rule_id=sysml-2.0:8.3.21.8:deriveRequirementDefinitionFramedConcern
rule_id=sysml-2.0:8.3.21.8:deriveRequirementDefinitionText
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package RequirementDefinitionDerivedFacts {
    part def Component;
    concern def Safety;
    requirement def Requirement {
        doc /* requirement definition text */
        subject subject : Component;
        actor actor : Component;
        frame concern concern : Safety;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.8:deriveRequirementDefinitionActorParameter") (source "RequirementDefinitionDerivedFacts::Requirement") (target "RequirementDefinitionDerivedFacts::Requirement::actor") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.8:deriveRequirementDefinitionSubjectParameter") (source "RequirementDefinitionDerivedFacts::Requirement") (target "RequirementDefinitionDerivedFacts::Requirement::subject") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.8:deriveRequirementDefinitionFramedConcern") (source "RequirementDefinitionDerivedFacts::Requirement") (target "RequirementDefinitionDerivedFacts::Requirement::concern") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.8:deriveRequirementDefinitionText") (source "RequirementDefinitionDerivedFacts::Requirement") (text " requirement definition text ") (outcome text)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_requirement_definition_derived_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:8ebfa51f464f0c73d04273b3008a4da6619af08dbfa16a75bcb1bde50b05851d") (contract-version "lossless-publication-completeness-v3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " requirement definition text "))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor"))) (kind requirement-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::concern"))) (kind frame) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::subject"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Safety"))) (kind concern-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")))))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::subject"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor"))) (target (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::subject"))) (target (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::subject"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor"))) (target (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::actors"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::concern"))) (target (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::concern"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::concerns"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::concern"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::concernChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::subject"))) (target (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Safety"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::subject")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor")))
      (featured-by (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement")))
      (type (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")) (source direct))
      (supertype (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::actors")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::concern")))
      (featured-by (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::concerns")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::concernChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::subject")))
      (featured-by (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement")))
      (type (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")) (source direct))
      (supertype (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Safety")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (range (start 6 22) (end 6 31)) (probe (position 6 22))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::actor"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")))))
    )
  )
  (query (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (range (start 5 26) (end 5 35)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Requirement::subject"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_definition_derived_facts.md") (qualified-name "RequirementDefinitionDerivedFacts::Component")))))
    )
  )
)
~~~
