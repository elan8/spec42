# META
~~~ini
description=Definition and Usage variant derivations retain the missing canonical VariantMembership identity
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionVariant
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionVariantMembership
rule_id=sysml-2.0:8.3.6.4:deriveUsageVariant
rule_id=sysml-2.0:8.3.6.4:deriveUsageVariantMembership
blocked_by=lowering-gap-definition-usage-variant-membership-identity
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
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionVariant") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionVariantMembership") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageVariant") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageVariantMembership") (source "Model::vehicle") (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_usage_variant_membership.md"
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f620ebd1d06ba3b8c3df4a5570c9ee4071389ad46d092a39cb15a8590f8a61f1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Vehicle::wheel"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Vehicle::wheel"))) (target (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Vehicle::wheel")))
      (featured-by (node (document "memory://snapshot/sysml_definition_usage_variant_membership.md") (qualified-name "Model::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
