# META
~~~ini
description=SysML 8.3.6.4 validateUsageVariationIsAbstract requires a variation Usage to be abstract
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.4 validateUsageVariationIsAbstract
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.6.4:validateUsageVariationIsAbstract
type=file
~~~
# SOURCE
~~~sysml
// The violating side has no textual counterpart: the grammar cannot spell a non-abstract
// variation, since `variation` itself implies isAbstract (SysML 8.3.6.2/8.3.6.4; the Pilot's
// DefinitionAdapter/UsageAdapter.postProcess set isAbstract for every variation).
package Variations {
    part def Base;
    part def Holder {
        // Conforming: a variation usage is abstract. `abstract` and `variation` are exclusive
        // alternatives of one BasicUsagePrefix slot, so isAbstract is implied by `variation`
        // rather than authored (the SMG publishes it under implied-modifiers).
        variation part good : Base;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_is_abstract.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_is_abstract.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:fb33d9c138460c4e6a65b0ca445e54fdbbb5c19452a0015d8547728f674fe5e2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::good"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation) (implied-modifiers abstract)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base") (variation true)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")))))
  )
  (relationships
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::good"))) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::good"))) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::good")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (range (start 9 30) (end 9 34)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")))))
    )
  )
)
~~~
