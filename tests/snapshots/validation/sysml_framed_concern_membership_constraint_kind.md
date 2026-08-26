# META
~~~ini
description=SysML 8.3.21.5 validateFramedConcernMembershipConstraintKind requires a FramedConcernMembership to have kind = requirement
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.5 validateFramedConcernMembershipConstraintKind
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.21.5:validateFramedConcernMembershipConstraintKind
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the frame keyword produces a FramedConcernMembership and fixes its kind to
// requirement at the same time. The pinned parser now carries a FrameConcernMember production,
// so this accepted side reaches semantics and reports nothing.
//
// The violating side has no textual counterpart: SysML concrete syntax offers no other kind for a
// framed concern, so the rule is observable only as the accepted side pinned here.
package Roles {
    part def Component;
    concern def Safety;
    requirement def Limited {
        subject item : Component;
        frame concern safety : Safety;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a40f509eb20e1dd92d4e35e261e0b9af87c97abb8e543693477475a34d1975d7") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::safety"))) (kind frame) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Safety"))) (kind concern-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::item"))) (target (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::item"))) (target (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::safety"))) (target (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::item")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::item")))
      (featured-by (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited")))
      (type (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::safety")))
      (featured-by (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (range (start 10 23) (end 10 32)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Limited::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_framed_concern_membership_constraint_kind.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
