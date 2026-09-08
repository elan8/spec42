# META
~~~ini
description=SysML 8.3.6.4 checkUsageVariationUsageSpecialization remains unresolved when a variant Usage's specialization target is unresolved
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
    part def Holder {
        variation part choice : Base {
            variant missing;
        }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "sysml-2.0:8.3.6.4:checkUsageVariationUsageSpecialization") (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 20) (end 4 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8506fa00583c81cc4e1026175de4a2ea7bc1d28dc9a6692cab45f8ca84bd6a61"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder::choice"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base") (variation true)))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (subsetting (reference "missing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder::choice"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder::choice"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder::choice"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder::choice"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder::choice")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder::choice")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Base")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (range (start 3 32) (end 3 36)) (probe (position 3 32))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Holder::choice"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (range (start 4 20) (end 4 27)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
