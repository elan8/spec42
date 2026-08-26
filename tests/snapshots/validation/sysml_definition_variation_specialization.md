# META
~~~ini
description=SysML 8.3.6.2 validateDefinitionVariationSpecialization forbids a variation Definition from specializing another variation Definition
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.6.2 validateDefinitionVariationSpecialization
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.6.2:validateDefinitionVariationSpecialization
blocked_by=parser-gap-78-variation-forms
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    abstract variation part def Root;
    abstract part def Plain;

    // Conforming: a variation definition specializing a non-variation definition.
    abstract variation part def Good specializes Plain;

    // Invalid: a variation definition specializing another variation definition.
    abstract variation part def Bad specializes Root;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_variation_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variation_specializes_variation")
        (source "semantic")
        (range (start 8 4) (end 8 53))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_variation_specialization.md"
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:0976a8e5dd4ea91cd3f5417cb5f21da81f2a5e4b22344ee884a5151e7c539d20") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Plain"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
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
