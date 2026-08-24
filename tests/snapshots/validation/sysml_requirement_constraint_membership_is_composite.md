# META
~~~ini
description=SysML 8.3.21.7 validateRequirementConstraintMembershipIsComposite requires the ownedConstraint of a RequirementConstraintMembership to be composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.7 validateRequirementConstraintMembershipIsComposite
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.21.7:validateRequirementConstraintMembershipIsComposite
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the required constraint below is owned compositely by the requirement definition,
// which is the only shape the require keyword produces.
//
// The violating side has no textual counterpart: SysML concrete syntax has no spelling for a
// referential required constraint, so the rule is observable only as the accepted side pinned
// here.
package Roles {
    part def Component;
    constraint def Bound;
    requirement def Limited {
        subject item : Component;
        require constraint limit : Bound;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:461935389d4d5282670769b5e349a2a909a21a0b8bc87b8fe659a329415a2563") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Bound"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::limit"))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bound")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::limit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Bound")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::item"))) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::limit"))) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Bound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::limit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::item"))) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::limit"))) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Bound")))
      (subtype (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::limit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::item")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::item")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited")))
      (type (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::limit")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited")))
      (type (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Bound")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Bound")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Bound")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (range (start 10 23) (end 10 32)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (range (start 11 35) (end 11 40)) (probe (position 11 35))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Limited::limit"))) (kind featureTyping) (ordinal 0) (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_constraint_membership_is_composite.md") (qualified-name "Roles::Bound")))))
    )
  )
)
~~~
