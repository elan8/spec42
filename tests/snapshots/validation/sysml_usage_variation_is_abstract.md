# META
~~~ini
description=SysML 8.3.6.4 validateUsageVariationIsAbstract requires a variation Usage to be abstract
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.4 validateUsageVariationIsAbstract
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=sysml-2.0:8.3.6.4:validateUsageVariationIsAbstract
blocked_by=abstract-syntax-nonrepresentable-abstract-variation
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    part def Base;
    part def Holder {
        // Conforming: RefPrefix variance is the exclusive slot `abstract` | `variation`
        // (SysML BNF 278). Pairing `abstract variation` has no textual spelling.
        variation part choices : Base;
    }
}
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:89a46b160621c649f4a3ccb1683b2f31094983f12034ce303ba463dd9f7cfb92"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::choices"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base") (variation true)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::choices"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")))))
  )
  (relationships
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::choices"))) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::choices"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::choices"))) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::choices")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::choices")))
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
  (query (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (range (start 5 33) (end 5 37)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::choices"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")))))
    )
  )
)
~~~
