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
blocked_by=parser-gap-78-variation-forms
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    part def Base;
    part def Holder {
        // Conforming: a variation usage declared abstract.
        abstract variation part good : Base;

        // Invalid: a variation usage must be abstract.
        variation part bad : Base;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_is_abstract.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variation_not_abstract")
        (source "semantic")
        (range (start 7 8) (end 7 34))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_is_abstract.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 4 8) (end 7 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:529deda5cf591fdbd4c074ec1b4f06d7af48a42104cc1afa6ca36f89721af6b3") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::bad"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base") (variation true)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")))))
  )
  (relationships
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::bad"))) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::bad"))) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::bad")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::bad")))
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
  (query (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (range (start 7 29) (end 7 33)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_is_abstract.md") (qualified-name "Variations::Base")))))
    )
  )
)
~~~
