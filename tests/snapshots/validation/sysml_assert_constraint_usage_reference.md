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
    part def Holder :> Library {
        // Conforming: the asserted feature is a constraint usage.
        assert limit;

        // Invalid: the asserted feature is a part usage, not a constraint usage.
        assert other;
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
        (range (start 12 15) (end 12 20))
        (related-information
          (related
            (uri "memory://snapshot/sysml_assert_constraint_usage_reference.md")
            (range (start 5 8) (end 5 31))
          )
        )
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
        (code "assert_target_invalid_kind")
        (source "semantic")
        (range (start 12 15) (end 12 20))
        (related-information
          (related
            (uri "memory://snapshot/sysml_assert_constraint_usage_reference.md")
            (range (start 5 8) (end 5 31))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2f39c4d359d53a79f21023f192c381cda9f59b60486d1202b31c5646735c67ec"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Library")))))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "limit")))))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "other")))))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bound")))))
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder"))) (kind specialization) (ordinal 0))
      (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder"))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder")))
      (supertype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder")))
      (effective-type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")) (source inherited) (from (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit"))))
      (supertype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder")))
      (effective-type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")) (source inherited) (from (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other"))))
      (supertype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library")))
      (subtype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit")))
      (featured-by (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library")))
      (type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Bound")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other")))
      (featured-by (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library")))
      (type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Component")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 1)))) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (range (start 7 23) (end 7 30)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Holder"))) (kind specialization) (ordinal 0) (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library")))))
    )
  )
  (query (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (range (start 9 15) (end 9 20)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::limit")))))
    )
  )
  (query (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (range (start 12 15) (end 12 20)) (probe (position 12 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (path (named (kind package) (name "Constraints")) (named (kind part-def) (name "Holder")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind referenceSubsetting) (ordinal 0) (authored-target "other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assert_constraint_usage_reference.md") (qualified-name "Constraints::Library::other")))))
    )
  )
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
