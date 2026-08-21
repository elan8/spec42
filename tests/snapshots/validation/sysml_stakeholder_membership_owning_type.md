# META
~~~ini
description=SysML 8.3.21.12 validateStakeholderMembershipOwningType requires the owningType of a StakeholderMembership to be a RequirementDefinition or a RequirementUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.12 validateStakeholderMembershipOwningType
type=file
skip_validation=the pinned parser has no production for this membership outside its permitted owning type, so the invalid model is reported as unexpected_keyword_in_scope and never reaches semantics
~~~
# SOURCE
~~~sysml
package Roles {
    part def Component;

    // Conforming: the stakeholder membership is owned by a requirement definition.
    requirement def Good {
        subject item : Component;
        stakeholder owner : Component;
    }

    // Invalid: the stakeholder membership is owned by a part definition.
    part def Bad {
        stakeholder owner : Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "stakeholder_membership_invalid_owner")
        (source "semantic")
        (range (start 10 4) (end 10 18))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 11 8) (end 12 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:e77a705804f1dc990a88b8b5d08ce7800510003d68523425f1ac1287d9e29d4a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Bad"))) (kind part-def) (membership (kind owning) (visibility default)))
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
  (query (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (range (start 5 23) (end 5 32)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (range (start 6 28) (end 6 37)) (probe (position 6 28))
    (reference (id (source (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Good::owner"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_stakeholder_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
