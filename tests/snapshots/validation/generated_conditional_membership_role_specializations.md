# META
~~~ini
description=Generated membership-role specializations select canonical FramedConcern, Actor, and Stakeholder role anchors
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.21.4:checkConcernUsageFramedConcernSpecialization
rule_id=sysml-2.0:8.3.11.3:checkPartUsageActorSpecialization
rule_id=sysml-2.0:8.3.11.3:checkPartUsageStakeholderSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package MembershipRoleSpecializations {
    part def Component;
    concern def Safety;

    requirement def RequirementCase {
        subject item : Component;
        frame concern safety : Safety;
        actor requirementActor : Component;
        stakeholder requirementStakeholder : Component;
    }

    case def CaseCase {
        actor caseActor : Component;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "MembershipRoleSpecializations::RequirementCase::safety") (target "Requirements::RequirementCheck::concerns") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "MembershipRoleSpecializations::RequirementCase::requirementActor") (target "Requirements::RequirementCheck::actors") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "MembershipRoleSpecializations::RequirementCase::requirementStakeholder") (target "Requirements::RequirementCheck::stakeholders") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "MembershipRoleSpecializations::CaseCase::caseActor") (target "Cases::Case::actors") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_membership_role_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:a14232fdad3bca43503ccb5895f6b20c3e34783e613d3e64489f6eb78eba2d4a") (contract-version "parser-owned-resolution-v2") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor"))) (kind requirement-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder"))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::safety"))) (kind frame) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Safety"))) (kind concern-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor"))) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::item"))) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor"))) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder"))) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase"))) (target (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor"))) (target (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor"))) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::item"))) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor"))) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::actors"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder"))) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::stakeholders"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::safety"))) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::safety"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::concerns"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::safety"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::concernChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Safety"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor")))
      (featured-by (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase")))
      (type (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::actors")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::item")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::item")))
      (featured-by (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase")))
      (type (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor")))
      (featured-by (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase")))
      (type (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder")))
      (featured-by (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase")))
      (type (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")) (scopes any))
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
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::stakeholders")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::safety")))
      (featured-by (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase")))
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
    (declaration (id (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Safety")))
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
  (query (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (range (start 12 26) (end 12 35)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::CaseCase::caseActor"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (range (start 5 23) (end 5 32)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (range (start 7 33) (end 7 42)) (probe (position 7 33))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementActor"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (range (start 8 45) (end 8 54)) (probe (position 8 45))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::RequirementCase::requirementStakeholder"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_membership_role_specializations.md") (qualified-name "MembershipRoleSpecializations::Component")))))
    )
  )
)
~~~
