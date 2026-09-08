# META
~~~ini
description=SysML 8.3.6.4 checkUsageVariationUsageSpecialization rejects a variant Usage that does not specialize its owning variation Usage
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:checkUsageVariationUsageSpecialization
coverage_role=secondary
~~~
# SOURCE
~~~sysml
package Model {
    part def Base;
    part def Other;
    part def Holder {
        variation part choice : Base {
            variant part option : Other;
        }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "sysml-2.0:8.3.6.4:checkUsageVariationUsageSpecialization") (outcome violated)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:15ec99379886e4e71b2f7fad34288d303b1b97f0207917b0ddcce387af07f34d"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base") (variation true)))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice::option"))) (kind part) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (featureTyping (reference "Other")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Other"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice::option"))) (kind featureTyping) (ordinal 0))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Other")))))
  )
  (relationships
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice::option"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice::option"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice::option")))
      (type (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Other")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Other")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Other")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Other")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice::option")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (range (start 4 32) (end 4 36)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (range (start 5 34) (end 5 39)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Holder::choice::option"))) (kind featureTyping) (ordinal 0) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_violating.md") (qualified-name "Model::Other")))))
    )
  )
)
~~~
