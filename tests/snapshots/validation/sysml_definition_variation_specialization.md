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
blocked_by=semantic-variation-specialization
type=file
~~~
# SOURCE
~~~sysml
package Variations {
    variation part def Root;
    abstract part def Plain;

    // Conforming: a variation definition specializing a non-variation definition.
    variation part def Good specializes Plain;

    // Invalid: a variation definition specializing another variation definition.
    variation part def Bad specializes Root;
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b75371dddbcd244802d103a9c09e6c5c6d15a030ed5049e488eba1f13b951455"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Bad"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Root")))))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Good"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Plain")))))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Plain"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Root"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Bad"))) (kind specialization) (ordinal 0))
      (authored-target "Root")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Root")))))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Good"))) (kind specialization) (ordinal 0))
      (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Plain")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Bad"))) (target (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Root"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Bad"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Good"))) (target (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Plain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Good"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Bad")))
      (supertype (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Root")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Good")))
      (supertype (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Plain")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Plain")))
      (subtype (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Good")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Root")))
      (subtype (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Bad")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_definition_variation_specialization.md") (range (start 8 39) (end 8 43)) (probe (position 8 39))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Bad"))) (kind specialization) (ordinal 0) (authored-target "Root")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Root")))))
    )
  )
  (query (document "memory://snapshot/sysml_definition_variation_specialization.md") (range (start 5 40) (end 5 45)) (probe (position 5 40))
    (reference (id (source (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Good"))) (kind specialization) (ordinal 0) (authored-target "Plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_definition_variation_specialization.md") (qualified-name "Variations::Plain")))))
    )
  )
)
~~~
