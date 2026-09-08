# META
~~~ini
description=SysML 8.3.6.4 checkUsageVariationUsageSpecialization accepts a variant Usage that specializes its owning variation Usage
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:checkUsageVariationUsageSpecialization
~~~
# SOURCE
~~~sysml
package Model {
    part def Base;
    part def Holder {
        variation part choice : Base {
            variant choice;
        }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "sysml-2.0:8.3.6.4:checkUsageVariationUsageSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_usage_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9e45c126a983020798c5433af4f34d3f761411649cfae096ee6f5c500a8031eb"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base") (variation true)))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (subsetting (reference "choice")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base")))))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "choice")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice")))))
  )
  (relationships
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice")))
      (featured-by (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder")))
      (type (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base")) (source inherited) (from (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice"))))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (range (start 3 32) (end 3 36)) (probe (position 3 32))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Base")))))
    )
  )
  (query (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (range (start 4 20) (end 4 26)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (path (named (kind package) (name "Model")) (named (kind part-def) (name "Holder")) (named (kind part) (name "choice")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "choice")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Holder::choice")))))
    )
  )
)
~~~
