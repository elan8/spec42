# META
~~~ini
description=Definition and Usage effective feature projections retain the first unavailable inherited FeatureMembership closure
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionDirectedUsage
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionUsage
rule_id=sysml-2.0:8.3.6.4:deriveUsageDirectedUsage
rule_id=sysml-2.0:8.3.6.4:deriveUsageUsage
libraries=none
~~~
# SOURCE
~~~sysml
package Model {
    part def Vehicle { part wheel; }
    part vehicle;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionDirectedUsage") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionUsage") (source "Model::Vehicle") (target "Model::Vehicle::wheel") (outcome resolved))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageDirectedUsage") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageUsage") (source "Model::vehicle") (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_usage_effective_feature_closure.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 1 23) (end 1 34))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 4) (end 2 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8eea75bb540f3f13159b0c14a6b7356b616b8c65daba9ca484e3b56287f984d3") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_effective_feature_closure.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_effective_feature_closure.md") (qualified-name "Model::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_effective_feature_closure.md") (qualified-name "Model::Vehicle::wheel"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_effective_feature_closure.md") (qualified-name "Model::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_definition_usage_effective_feature_closure.md") (qualified-name "Model::Vehicle::wheel"))) (target (node (document "memory://snapshot/sysml_definition_usage_effective_feature_closure.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_effective_feature_closure.md") (qualified-name "Model::Vehicle::wheel")))
      (featured-by (node (document "memory://snapshot/sysml_definition_usage_effective_feature_closure.md") (qualified-name "Model::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
