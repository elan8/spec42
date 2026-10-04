# META
~~~ini
description=SysML 8.3.21.12 validateStakeholderMembershipOwningType requires the owningType of a StakeholderMembership to be a RequirementDefinition or a RequirementUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.12 validateStakeholderMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
evidence_reference=file:tests/snapshots/validation/sysml_grammar_restricted_membership_owners.md
rule_id=sysml-2.0:8.3.21.12:validateStakeholderMembershipOwningType
type=file
~~~
# SOURCE
~~~sysml
// The violating side has no textual counterpart: the grammar admits this element only in
// requirement bodies (StakeholderMember in RequirementBodyItem), whose owners all satisfy the rule.
// sysml_grammar_restricted_membership_owners.md pins the parser rejecting a forbidden owner.
package Roles {
    part def Component;

    // Conforming: the stakeholder membership is owned by a requirement definition.
    requirement def Good {
        subject item : Component;
        stakeholder owner : Component;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:41d853586f44062e2cc8ca24a179c9ae34828db68bdcebfb081870d349f05c4e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::owner"))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::owner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::owner"))) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::owner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::owner"))) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::item")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::owner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::item")))
      (featured-by (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::owner")))
      (featured-by (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (range (start 8 23) (end 8 32)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (range (start 9 28) (end 9 37)) (probe (position 9 28))
    (reference (id (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::owner"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
