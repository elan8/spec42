# META
~~~ini
description=SysML 8.3.6.4 checkUsageVariationUsageTypeFeaturing requires a Usage with owningVariationUsage to share that usage's featuringTypes
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:checkUsageVariationUsageTypeFeaturing
blocked_by=parser-gap-78-variation-forms
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    abstract variation part def Root;
    abstract part def Plain;
    part def Holder {
        abstract variation part good : Plain;
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
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 1 4) (end 2 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 1 4) (end 2 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:fa74ddd292356651019b83c615f11004f3035d2f467240dfd09c2fffa2e6acf2") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_type_featuring.md") (qualified-name "Variations::Plain"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
