# META
~~~ini
description=SysML 8.3.21.11 validateSubjectMembershipOwningType requires the owningType of a SubjectMembership to be a RequirementDefinition, RequirementUsage, CaseDefinition or CaseUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.11 validateSubjectMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=sysml-2.0:8.3.21.11:validateSubjectMembershipOwningType
blocked_by=abstract-syntax-invalid-membership-owner
type=file
~~~
# SOURCE
~~~sysml
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e113608359f7ad4ba09fe8594fe7e89b2690aff54bbf5f173c0cf8b76c305527") (contract-version "semantic-metadata-projection-v6"))
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
  (query (document "memory://snapshot/sysml_subject_membership_owning_type.md") (range (start 5 23) (end 5 32)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_subject_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
