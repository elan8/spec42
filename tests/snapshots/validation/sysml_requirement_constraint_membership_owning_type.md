# META
~~~ini
description=SysML 8.3.21.7 validateRequirementConstraintMembershipOwningType requires the owningType of a RequirementConstraintMembership to be a RequirementDefinition or a RequirementUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.7 validateRequirementConstraintMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.21.7:validateRequirementConstraintMembershipOwningType
blocked_by=parser-gap-74-require-constraint-membership
type=file
~~~
# SOURCE
~~~sysml
package Roles {
    part def Component;
    constraint def Bound;

    // Conforming: the constraint membership is owned by a requirement definition.
    requirement def Good {
        subject item : Component;
        require constraint limit : Bound;
    }

    // Invalid: the constraint membership is owned by a part definition.
    part def Bad {
        require constraint limit : Bound;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "requirement_constraint_invalid_owner")
        (source "semantic")
        (range (start 11 4) (end 11 18))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 12 8) (end 13 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:90e2c9c2eacf75643a9cace9f8936678aba596833eaaeda6c2e11631e82bbf81") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Bad"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Bound"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::limit"))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bound")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::limit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Bound")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::limit"))) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Bound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::limit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::limit"))) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Bound")))
      (subtype (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::limit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::item")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::item")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::limit")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Bound")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Bound")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Bound")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (range (start 6 23) (end 6 32)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (range (start 7 35) (end 7 40)) (probe (position 7 35))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Good::limit"))) (kind featureTyping) (ordinal 0) (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_owning_type.md") (qualified-name "Roles::Bound")))))
    )
  )
)
~~~
