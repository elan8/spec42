# META
~~~ini
description=SysML 8.3.6.4 checkUsageVariationUsageTypeFeaturing requires a Usage with owningVariationUsage to share that usage's featuringTypes
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:checkUsageVariationUsageTypeFeaturing
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    variation part def Root;
    abstract part def Plain;
    part def Holder {
        variation part good : Plain;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source "Variations::Holder::good")
    (target "Variations::Holder")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e5526059c961fa2c3e26cfae7487bee893a28e045b886ecc229273eec8dd24a3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder::good"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Plain") (variation true)))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Plain"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Root"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Plain")))))
  )
  (relationships
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder::good"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Plain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder::good"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Plain")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Plain")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Plain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Plain")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder::good")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (range (start 4 30) (end 4 35)) (probe (position 4 30))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Plain")))))
    )
  )
)
~~~
