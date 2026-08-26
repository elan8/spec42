# META
~~~ini
description=SysML 8.3.6.3 validateReferenceUsageIsReference requires a ReferenceUsage to be referential
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.3 validateReferenceUsageIsReference
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.6.3:validateReferenceUsageIsReference
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the ref keyword is what produces a ReferenceUsage, and it sets isReference at the
// same time.
//
// The violating side has no textual counterpart: SysML concrete syntax has no spelling that
// produces a non-referential ReferenceUsage, so the rule is observable only as the accepted side
// pinned here.
package References {
    part def Base;
    part def Holder {
        ref part referenced : Base;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_reference_usage_is_reference.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_reference_usage_is_reference.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:473ad2b1f4d44d2345599285ca1f06dda459847bd8f4272254f6ab094b176a0b") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder::referenced"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder::referenced"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder::referenced"))) (target (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder::referenced"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder::referenced"))) (target (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Base")))
      (subtype (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder::referenced")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder::referenced")))
      (featured-by (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Base")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_reference_usage_is_reference.md") (range (start 9 30) (end 9 34)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Holder::referenced"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_reference_usage_is_reference.md") (qualified-name "References::Base")))))
    )
  )
)
~~~
