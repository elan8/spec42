# META
~~~ini
description=SysML 8.3.6.4 checkUsageVariationDefinitionSpecialization rejects a variant Usage that does not specialize its owning variation Definition
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:checkUsageVariationDefinitionSpecialization
coverage_role=secondary
~~~
# SOURCE
~~~sysml
package Model {
    part def Other;
    variation part def Choice {
        variant part option : Other;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "sysml-2.0:8.3.6.4:checkUsageVariationDefinitionSpecialization") (outcome violated)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:29d7f9ae56acd0544f4c7dde0fb5431c83e44730f8c4a45f38a2dd29ac5f0f0e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Choice"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Choice::option"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Other")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Other"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Choice::option"))) (kind featureTyping) (ordinal 0))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Other")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Choice::option"))) (target (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Choice::option"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Choice::option")))
      (type (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Other")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Other")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Other")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Other")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Choice::option")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (range (start 3 30) (end 3 35)) (probe (position 3 30))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Choice::option"))) (kind featureTyping) (ordinal 0) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_definition_specialization_violating.md") (qualified-name "Model::Other")))))
    )
  )
)
~~~
