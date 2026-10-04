# META
~~~ini
description=SysML 8.3.22.4 validateObjectiveMembershipOwningType requires the owningType of an ObjectiveMembership to be a CaseDefinition or a CaseUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.22.4 validateObjectiveMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
evidence_reference=file:tests/snapshots/validation/sysml_grammar_restricted_membership_owners.md
rule_id=sysml-2.0:8.3.22.4:validateObjectiveMembershipOwningType
type=file
~~~
# SOURCE
~~~sysml
// The violating side has no textual counterpart: the grammar admits this element only in
// case bodies (ObjectiveMember in CaseBodyItem), whose owners all satisfy the rule.
// sysml_grammar_restricted_membership_owners.md pins the parser rejecting a forbidden owner.
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:517dbc7ce31b793163eb13ea5d18bfc674b51f6e2980c500385231b3451330aa"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::achieved"))) (kind objective-requirement) (membership (kind feature) (visibility default)))
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
  (query (document "memory://snapshot/sysml_objective_membership_owning_type.md") (range (start 8 23) (end 8 32)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_objective_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
