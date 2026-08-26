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
blocked_by=parser-gap-78-variation-forms
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    // Conforming: a variation definition declared abstract.
    abstract variation part def Good;

    // Invalid: a variation definition must be abstract.
    variation part def Bad;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_variation_is_abstract.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variation_not_abstract")
        (source "semantic")
        (range (start 5 4) (end 5 27))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_variation_is_abstract.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 2 4) (end 5 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:339ed252a4fc80dda78aeb5615047122ab1f1d4582be1ed829ca961ec36cbce9") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_is_abstract.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_is_abstract.md") (qualified-name "Variations::Bad"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
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
