# META
~~~ini
description=RequirementUsage exact derived properties project canonical membership roles and documentation bodies
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.21.9:deriveRequirementUsageActorParameter
rule_id=sysml-2.0:8.3.21.9:deriveRequirementUsageSubjectParameter
rule_id=sysml-2.0:8.3.21.9:deriveRequirementUsageFramedConcern
rule_id=sysml-2.0:8.3.21.9:deriveRequirementUsageText
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package RequirementUsageDerivedFacts {
    part def Component;
    concern def Safety;
    requirement Requirement {
        doc /* requirement usage text */
        subject 'subject' : Component;
        actor 'actor' : Component;
        frame concern 'concern' : Safety;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.9:deriveRequirementUsageActorParameter") (source "RequirementUsageDerivedFacts::Requirement") (target "RequirementUsageDerivedFacts::Requirement::actor") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.9:deriveRequirementUsageSubjectParameter") (source "RequirementUsageDerivedFacts::Requirement") (target "RequirementUsageDerivedFacts::Requirement::subject") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.9:deriveRequirementUsageFramedConcern") (source "RequirementUsageDerivedFacts::Requirement") (target "RequirementUsageDerivedFacts::Requirement::concern") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.9:deriveRequirementUsageText") (source "RequirementUsageDerivedFacts::Requirement") (text "requirement usage text ") (outcome text)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_requirement_usage_derived_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:404a2642c546e7d58fd54c2706aa75656aa40a7d57a6800731d0ed3122ada07b") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (path (named (kind package) (name "RequirementUsageDerivedFacts")) (named (kind requirement) (name "Requirement")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "requirement usage text "))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor"))) (kind requirement-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern"))) (kind frame) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Safety")))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::subject"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Safety"))) (kind concern-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")))))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern"))) (kind featureTyping) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Safety")))))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::subject"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor"))) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern"))) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::subject"))) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::subject"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor"))) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::actors"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern"))) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::concerns"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::concernChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::subject"))) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::subject"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::subj"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Safety"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::subject")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor")))
      (featured-by (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement")))
      (type (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::actors"))))
      (supertype (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::actors")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern")))
      (featured-by (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement")))
      (type (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Safety")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Safety")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::concernChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
      (supertype (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Safety")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::concerns")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::concernChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::subject")))
      (featured-by (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement")))
      (type (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::subj"))))
      (supertype (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::subj")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Safety")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (range (start 6 24) (end 6 33)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::actor"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")))))
    )
  )
  (query (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (range (start 7 34) (end 7 40)) (probe (position 7 34))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::concern"))) (kind featureTyping) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Safety")))))
    )
  )
  (query (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (range (start 5 28) (end 5 37)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Requirement::subject"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_usage_derived_facts.md") (qualified-name "RequirementUsageDerivedFacts::Component")))))
    )
  )
)
~~~
