# META
~~~ini
description=SysML 8.3.22.4 validateObjectiveMembershipOwningType requires the owningType of an ObjectiveMembership to be a CaseDefinition or a CaseUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.22.4 validateObjectiveMembershipOwningType
type=file
skip_validation=the pinned parser has no production for this membership outside its permitted owning type, so the invalid model is reported as unexpected_keyword_in_scope and never reaches semantics
~~~
# SOURCE
~~~sysml
package Roles {
    part def Component;

    // Conforming: the objective membership is owned by a case definition.
    case def Good {
        subject item : Component;
        objective achieved;
    }

    // Invalid: the objective membership is owned by a part definition.
    part def Bad {
        objective achieved;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_objective_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "objective_membership_invalid_owner")
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
  (document "memory://snapshot/sysml_objective_membership_owning_type.md"
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:62547c46ed43a7679aa19b0a41d68186e5272407972d7205a985e45ecfa3a297") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Bad"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::achieved"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (target (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::item")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::achieved")))
      (featured-by (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::item")))
      (featured-by (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_objective_membership_owning_type.md") (range (start 5 23) (end 5 32)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
