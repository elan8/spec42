# META
~~~ini
description=SysML 8.3.6.4 checkUsageVariationDefinitionSpecialization accepts a variant Usage that specializes its owning variation Definition
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:checkUsageVariationDefinitionSpecialization
~~~
# SOURCE
~~~sysml
package Model {
    variation part def Choice {
        variant part option : Option;
    }
    part def Option :> Choice;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "sysml-2.0:8.3.6.4:checkUsageVariationDefinitionSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_definition_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3027d9717dacea9fa3d408407f403e89a0af8c1395308228f4b290bb1626422c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice::option"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Option")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Choice")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice::option"))) (kind featureTyping) (ordinal 0))
      (authored-target "Option")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option"))) (kind specialization) (ordinal 0))
      (authored-target "Choice")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice::option"))) (target (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice::option"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option"))) (target (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice::option")))
      (type (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option")))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice::option")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (range (start 2 30) (end 2 36)) (probe (position 2 30))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice::option"))) (kind featureTyping) (ordinal 0) (authored-target "Option")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (range (start 4 23) (end 4 29)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Option"))) (kind specialization) (ordinal 0) (authored-target "Choice")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_definition_specialization.md") (qualified-name "Model::Choice")))))
    )
  )
)
~~~
