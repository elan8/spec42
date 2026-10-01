# META
~~~ini
description=SysML 8.3.6.2 validateDefinitionVariationIsAbstract requires a variation Definition to be abstract
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.2 validateDefinitionVariationIsAbstract
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.6.2:validateDefinitionVariationIsAbstract
type=file
~~~
# SOURCE
~~~sysml
// The violating side has no textual counterpart: the grammar cannot spell a non-abstract
// variation, since `variation` itself implies isAbstract (SysML 8.3.6.2/8.3.6.4; the Pilot's
// DefinitionAdapter/UsageAdapter.postProcess set isAbstract for every variation).
package Variations {
    // Conforming: a variation definition is abstract. `abstract` and `variation` are exclusive
    // alternatives of one BasicDefinitionPrefix slot, so isAbstract is implied by `variation`
    // rather than authored (the SMG publishes it under implied-modifiers).
    variation part def Good;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_variation_is_abstract.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_variation_is_abstract.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a8fe1860f0307efabadb328324d18e4198a67c7228e4572fd0a6972cbde99b9f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_is_abstract.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_is_abstract.md") (qualified-name "Variations::Good"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation) (implied-modifiers abstract)))
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
