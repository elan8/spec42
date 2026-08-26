# META
~~~ini
description=SysML 8.3.24.2 validateRequirementVerificationMembershipOwningType requires the owningType of a RequirementVerificationMembership to be a RequirementUsage owned by an ObjectiveMembership
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.24.2 validateRequirementVerificationMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.24.2:validateRequirementVerificationMembershipOwningType
blocked_by=parser-gap-79-membership-owner-forms
type=file
~~~
# SOURCE
~~~sysml
package Verification {
    part def Component;
    requirement def Limit;

    // Conforming: the verification membership sits inside the objective of a verification case.
    verification def Good {
        subject item : Component;
        objective {
            verify requirement limit : Limit;
        }
    }

    // Invalid: the verification membership is owned by a part definition.
    part def Bad {
        verify requirement limit : Limit;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "verification_membership_invalid_owner")
        (source "semantic")
        (range (start 13 4) (end 13 18))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 14 8) (end 15 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:72febbfed1d4fce8abd5bf1ef64a07d29e1f21866c12cd1c795330cbdf8a19d4") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Bad"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective::limit"))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Limit")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Limit"))) (kind requirement-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective::limit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Limit")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::item"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective::limit"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective::limit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::item"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective::limit"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Component")))
      (subtype (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::item")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::item")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good")))
      (type (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective::limit")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective")))
      (type (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Limit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Limit")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Limit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Limit")))
      (subtype (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective::limit")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (range (start 6 23) (end 6 32)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (range (start 8 39) (end 8 44)) (probe (position 8 39))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Good::objective::limit"))) (kind featureTyping) (ordinal 0) (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_owning_type.md") (qualified-name "Verification::Limit")))))
    )
  )
)
~~~
