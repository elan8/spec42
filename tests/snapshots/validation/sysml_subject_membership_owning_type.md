# META
~~~ini
description=SysML 8.3.21.11 validateSubjectMembershipOwningType requires the owningType of a SubjectMembership to be a RequirementDefinition, RequirementUsage, CaseDefinition or CaseUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.11 validateSubjectMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
evidence_reference=file:tests/snapshots/validation/sysml_grammar_restricted_membership_owners.md
rule_id=sysml-2.0:8.3.21.11:validateSubjectMembershipOwningType
type=file
~~~
# SOURCE
~~~sysml
// The violating side has no textual counterpart: the grammar admits this element only in
// requirement and case bodies (SubjectMember in RequirementBodyItem and CaseBodyItem), whose owners all satisfy the rule.
// sysml_grammar_restricted_membership_owners.md pins the parser rejecting a forbidden owner.
package Roles {
    part def Component;

    // Conforming: the subject membership is owned by a requirement definition.
    requirement def Good {
        subject item : Component;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_subject_membership_owning_type.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:584f7ace33c840cdc37e9a9b8bcb99424b4d6c632c2050ecdd31ec983d0c4558"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (target (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (target (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good::item")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good::item")))
      (featured-by (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_subject_membership_owning_type.md") (range (start 8 23) (end 8 32)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
