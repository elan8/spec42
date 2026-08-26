# META
~~~ini
description=SysML 8.3.22.4 validateObjectiveMembershipOwningType requires the owningType of an ObjectiveMembership to be a CaseDefinition or a CaseUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.22.4 validateObjectiveMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=sysml-2.0:8.3.22.4:validateObjectiveMembershipOwningType
blocked_by=abstract-syntax-invalid-membership-owner
type=file
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
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_objective_membership_owning_type.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1dbac69c66f5c301b2d031d6f843a3d07ef042c79e2af5eb5f01fa6279125bc7") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
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
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::achieved"))) (target (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (target (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good"))) (provenance implied))
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
