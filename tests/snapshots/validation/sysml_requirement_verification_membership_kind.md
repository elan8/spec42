# META
~~~ini
description=SysML 8.3.24.2 validateRequirementVerificationMembershipKind requires a RequirementVerificationMembership to have kind = requirement
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.24.2 validateRequirementVerificationMembershipKind
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.24.2:validateRequirementVerificationMembershipKind
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the verify keyword produces a RequirementVerificationMembership and fixes its kind
// to requirement at the same time.
//
// The violating side has no textual counterpart: SysML concrete syntax offers no other kind for a
// verified requirement, so the rule is observable only as the accepted side pinned here.
package Verification {
    part def Component;
    requirement def Limit;
    verification def Check {
        subject item : Component;
        objective {
            verify requirement limit : Limit;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_verification_membership_kind.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_verification_membership_kind.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0a602cf69bd1c4587cc3a3c7f00877baa8ccb3b6a822888ec06c60044900c4e7") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::item"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective::limit"))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Limit")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Limit"))) (kind requirement-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective::limit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Limit")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::item"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective::limit"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective::limit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::item"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective::limit"))) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::item")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check")))
      (type (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective::limit")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective")))
      (type (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Limit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Limit")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Limit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Component")))
      (subtype (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::item")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Limit")))
      (subtype (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective::limit")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (range (start 9 23) (end 9 32)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::item"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (range (start 11 39) (end 11 44)) (probe (position 11 39))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Check::objective::limit"))) (kind featureTyping) (ordinal 0) (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_verification_membership_kind.md") (qualified-name "Verification::Limit")))))
    )
  )
)
~~~
