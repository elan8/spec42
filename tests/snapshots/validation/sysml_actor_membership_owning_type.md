# META
~~~ini
description=SysML 8.3.21.2 validateActorMembershipOwningType requires the owningType of an ActorMembership to be a RequirementDefinition, RequirementUsage, CaseDefinition or CaseUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.21.2 validateActorMembershipOwningType
type=file
skip_validation=the pinned parser has no production for this membership outside its permitted owning type, so the invalid model is reported as unexpected_keyword_in_scope and never reaches semantics
~~~
# SOURCE
~~~sysml
package Roles {
    part def Component;

    // Conforming: the actor membership is owned by a requirement definition.
    requirement def Good {
        subject item : Component;
        actor operator : Component;
    }

    // Invalid: the actor membership is owned by a part definition.
    part def Bad {
        actor operator : Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_actor_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "actor_membership_invalid_owner")
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
  (document "memory://snapshot/sysml_actor_membership_owning_type.md"
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:54a2e812f69467b3fe7135cc76a570082b8153f814475dfe9cc7b18acf42538b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Bad"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::operator"))) (kind requirement-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::operator"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (target (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::operator"))) (target (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::operator"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")))
      (subtype (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::item")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::operator")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::item")))
      (featured-by (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::operator")))
      (featured-by (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good")))
      (type (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_actor_membership_owning_type.md") (range (start 5 23) (end 5 32)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_actor_membership_owning_type.md") (range (start 6 25) (end 6 34)) (probe (position 6 25))
    (reference (id (source (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Good::operator"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_actor_membership_owning_type.md") (qualified-name "Roles::Component")))))
    )
  )
)
~~~
