# META
~~~ini
description=SysML 8.3.20.2 validateAssertConstraintUsageReference requires the featureTarget of the referencedFeature of an AssertConstraintUsage ownedReferenceSubsetting to be a ConstraintUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.20.2 validateAssertConstraintUsageReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.20.2:validateAssertConstraintUsageReference
blocked_by=lowering-part-definition-members
type=file
~~~
# SOURCE
~~~sysml
package Constraints {
    part def Component;
    constraint def Bound;
    part def Library {
        constraint limit : Bound;
        part other : Component;
    }
    part def Holder {
        // Conforming: the asserted feature is a constraint usage.
        assert Library::limit;

        // Invalid: the asserted feature is a part usage, not a constraint usage.
        assert Library::other;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assert_constraint_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "assert_target_invalid_kind")
        (source "semantic")
        (range (start 12 8) (end 12 30))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assert_constraint_usage_reference.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 9 8) (end 9 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 12 8) (end 12 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:ad01defc02fa6cb9508747cb510f66b14460b69deebb41366aee698741b4f481") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bound")))))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")))
      (subtype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")))
      (subtype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit")))
      (featured-by (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library")))
      (type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other")))
      (featured-by (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library")))
      (type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (range (start 4 27) (end 4 32)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (kind featureTyping) (ordinal 0) (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")))))
    )
  )
  (query (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (range (start 5 21) (end 5 30)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")))))
    )
  )
)
~~~
