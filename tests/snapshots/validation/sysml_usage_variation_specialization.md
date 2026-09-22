# META
~~~ini
description=SysML 8.3.6.4 validateUsageVariationSpecialization forbids a variation Usage from specializing any variation Definition or Usage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.4 validateUsageVariationSpecialization
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.6.4:validateUsageVariationSpecialization
blocked_by=semantic-variation-specialization
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    variation part def Root;
    abstract part def Plain;
    part def Holder {
        // Conforming: a variation usage typed by a non-variation definition.
        variation part good : Plain;

        // Invalid: a variation usage typed by a variation definition.
        variation part bad : Root;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variation_specializes_variation")
        (source "semantic")
        (range (start 8 8) (end 8 53))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bf2a0489bf43c75b4c28d21bc9fa77de2e12784aa97e149b5b5d7dff1f318882"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::bad"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Root") (variation true)))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::good"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Plain") (variation true)))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Plain"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Root"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "Root")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Root")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Plain")))))
  )
  (relationships
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::bad"))) (target (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Root"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::good"))) (target (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Plain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::bad"))) (target (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::good"))) (target (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Root")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Root")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Root")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Plain")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Plain")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Plain")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Plain")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::good")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Root")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::bad")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_specialization.md") (range (start 8 29) (end 8 33)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "Root")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Root")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_specialization.md") (range (start 5 30) (end 5 35)) (probe (position 5 30))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_specialization.md") (qualified-name "Variations::Plain")))))
    )
  )
)
~~~
