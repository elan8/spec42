# META
~~~ini
description=SysML 8.3.6.4 validateUsageIsReferential requires a Usage that is directed, is an end feature, or has no featuringTypes to be referential
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.4 validateUsageIsReferential
source_expectation=accepted
rule_family=validate
expectation=by_construction
evidence_reference=file:tests/snapshots/validation/sysml_usage_is_reference_derivation.md
rule_id=sysml-2.0:8.3.6.4:validateUsageIsReferential
type=file
~~~
# SOURCE
~~~sysml
// The violating side has no model: Usage::isReference is derived (not isComposite), and a
// directed usage, an end usage, or a usage with no featuring type is never composite (the Pilot's
// UsageAdapter.postProcess, which @satisfies this rule). sysml_usage_is_reference_derivation.md
// pins the derived isReference of each such usage.
package References {
    part def Base;
    part def Holder {
        // Conforming: a directed usage declared referential.
        in ref part directedReference : Base;

        // Also conforming: a directed usage without `ref` is referential by derivation.
        in part directedComposite : Base;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_is_referential.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3c8d7c78e6ccc8a16f73527fc2e9b51e991b0a1bba2236f717bee6660827b78f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedComposite"))) (kind part) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedReference"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedComposite"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedReference"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedComposite"))) (target (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedComposite"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedReference"))) (target (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedReference"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedComposite"))) (target (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedReference"))) (target (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")))
      (subtype (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedComposite")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedReference")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedComposite")))
      (featured-by (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedReference")))
      (featured-by (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_is_referential.md") (range (start 11 36) (end 11 40)) (probe (position 11 36))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedComposite"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_is_referential.md") (range (start 8 40) (end 8 44)) (probe (position 8 40))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Holder::directedReference"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_is_referential.md") (qualified-name "References::Base")))))
    )
  )
)
~~~
