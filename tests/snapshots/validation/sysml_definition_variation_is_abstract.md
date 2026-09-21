# META
~~~ini
description=SysML 8.3.6.2 validateDefinitionVariationIsAbstract requires a variation Definition to be abstract
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.2 validateDefinitionVariationIsAbstract
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=sysml-2.0:8.3.6.2:validateDefinitionVariationIsAbstract
blocked_by=abstract-syntax-nonrepresentable-abstract-variation
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    // Conforming: BasicDefinitionPrefix is the exclusive slot `abstract` | `variation`
    // (SysML BNF 219). Pairing `abstract variation` has no textual spelling, so a distinct
    // non-abstract variation cannot be authored either.
    variation part def Choices;
}
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5c9f236e2e13c994e5218227b6bc0e56fa32b13e3b514d5d84c248b0b2910df4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_is_abstract.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_is_abstract.md") (qualified-name "Variations::Choices"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
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
