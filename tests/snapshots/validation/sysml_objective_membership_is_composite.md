# META
~~~ini
description=SysML 8.3.22.4 validateObjectiveMembershipIsComposite requires the ownedObjectiveRequirement of an ObjectiveMembership to be composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.22.4 validateObjectiveMembershipIsComposite
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.22.4:validateObjectiveMembershipIsComposite
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the objective requirement below is owned compositely by the case definition, which
// is the only shape the objective keyword produces.
//
// The violating side has no textual counterpart: SysML concrete syntax has no spelling for a
// referential objective requirement, so the rule is observable only as the accepted side pinned
// here.
package Roles {
    part def Component;
    case def Analysis {
        subject item : Component;
        objective achieved;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_objective_membership_is_composite.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_objective_membership_is_composite.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:eb7dda169512cf200691831b68eef52e22ac33c7b23e6ac7ba2f9da53d53b04f") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::achieved"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::item"))) (target (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::achieved"))) (target (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::item"))) (target (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::achieved")))
      (featured-by (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::item")))
      (featured-by (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis")))
      (type (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::item")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_objective_membership_is_composite.md") (range (start 9 23) (end 9 32)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Analysis::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_objective_membership_is_composite.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
