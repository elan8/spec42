# META
~~~ini
description=Systems Definition and Usage exact derived collections consume canonical direct ownership, membership, kind, and modifier facts while unavailable inherited and time-variation inputs remain typed prerequisite outcomes
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedAction
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedAllocation
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedAnalysisCase
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedAttribute
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedCalculation
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedCase
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedConcern
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedConnection
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedConstraint
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedEnumeration
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedFlow
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedInterface
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedItem
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedMetadata
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedOccurrence
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedPart
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedPort
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedReference
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedRendering
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedRequirement
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedState
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedTransition
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedUsage
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedUseCase
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedVerificationCase
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedView
rule_id=sysml-2.0:8.3.6.2:deriveDefinitionOwnedViewpoint
rule_id=sysml-2.0:8.3.6.4:deriveUsageIsReference
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedAction
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedAllocation
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedAnalysisCase
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedAttribute
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedCalculation
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedCase
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedConcern
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedConnection
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedConstraint
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedEnumeration
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedFlow
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedInterface
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedItem
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedMetadata
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedOccurrence
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedPart
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedPort
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedReference
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedRendering
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedRequirement
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedState
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedTransition
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedUsage
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedUseCase
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedVerificationCase
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedView
rule_id=sysml-2.0:8.3.6.4:deriveUsageNestedViewpoint
libraries=none
~~~
# SOURCE
~~~sysml
package Model {
    part def Vehicle {
        part wheel;
        action service;
    }
    part vehicle;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedAction") (source "Model::Vehicle") (target "Model::Vehicle::service") (outcome resolved))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedAllocation") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedAnalysisCase") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedAttribute") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedCalculation") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedCase") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedConcern") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedConnection") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedConstraint") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedEnumeration") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedFlow") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedInterface") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedItem") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedMetadata") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedOccurrence") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedPart") (source "Model::Vehicle") (target "Model::Vehicle::wheel") (outcome resolved))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedPort") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedReference") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedRendering") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedRequirement") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedState") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedTransition") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedUsage") (source "Model::Vehicle") (target "Model::Vehicle::wheel") (outcome resolved))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedUseCase") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedVerificationCase") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedView") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.2:deriveDefinitionOwnedViewpoint") (source "Model::Vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageIsReference") (source "Model::vehicle") (outcome false))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedAction") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedAllocation") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedAnalysisCase") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedAttribute") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedCalculation") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedCase") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedConcern") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedConnection") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedConstraint") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedEnumeration") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedFlow") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedInterface") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedItem") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedMetadata") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedOccurrence") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedPart") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedPort") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedReference") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedRendering") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedRequirement") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedState") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedTransition") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedUsage") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedUseCase") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedVerificationCase") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedView") (source "Model::vehicle") (outcome absent))
  (definition-usage-derived (rule_id "sysml-2.0:8.3.6.4:deriveUsageNestedViewpoint") (source "Model::vehicle") (outcome absent))
  )
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_definition_usage_derived_collections.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 8) (end 2 19))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 4) (end 5 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4893fde4d413d3a12c21a728e8a1e19ef28f7492c6476562ebcb82fb5e7b152e") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle::service"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle::wheel"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle::service"))) (target (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle::wheel"))) (target (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle::service")))
      (featured-by (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle::wheel")))
      (featured-by (node (document "memory://snapshot/sysml_definition_usage_derived_collections.md") (qualified-name "Model::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
