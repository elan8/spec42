# META
~~~ini
description=Calculation, allocation, rendering, and view definitions project their owned usages of the matching kind
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.19.2:deriveCalculationUsageCalculation
rule_id=sysml-2.0:8.3.15.2:deriveAllocationDefinitionAllocation
rule_id=sysml-2.0:8.3.26.5:deriveRenderingDefinitionRendering
rule_id=sysml-2.0:8.3.26.7:deriveViewDefinitionView
libraries=none
~~~
# SOURCE
~~~sysml
package Projections {
    calc def Formula {
        calc step;
    }
    allocation def Map {
        allocate source to target;
    }
    rendering def Picture {
        render rendering detail;
    }
    view def Scene {
        view inset;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.15.2:deriveAllocationDefinitionAllocation") (source "Projections::Map") (target "Projections::Map::") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.19.2:deriveCalculationUsageCalculation") (source "Projections::Formula") (target "Projections::Formula::step") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.5:deriveRenderingDefinitionRendering") (source "Projections::Picture") (target "Projections::Picture::detail") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.7:deriveViewDefinitionView") (source "Projections::Scene") (target "Projections::Scene::inset") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_owned_usage_projections.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 17) (end 5 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 27) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "view_rendering_invalid_owner")
        (source "semantic")
        (range (start 8 8) (end 8 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:71d2dc2398cfc6e39a5982e6bf13338805190a9d56c90c02e6702b0f04123043"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Formula"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Formula::step"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Map"))) (kind allocation-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (path (named (kind package) (name "Projections")) (named (kind allocation-def) (name "Map")) (anonymous (kind allocate) (ordinal 0))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "source")) (allocateTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Picture"))) (kind rendering-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Picture::detail"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)))
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Scene"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Scene::inset"))) (kind view) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_owned_usage_projections.md") (path (named (kind package) (name "Projections")) (named (kind allocation-def) (name "Map")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_owned_usage_projections.md") (path (named (kind package) (name "Projections")) (named (kind allocation-def) (name "Map")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Formula::step"))) (target (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Formula"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_owned_usage_projections.md") (path (named (kind package) (name "Projections")) (named (kind allocation-def) (name "Map")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Map"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Picture::detail"))) (target (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Picture"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Scene::inset"))) (target (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Scene"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Formula::step")))
      (featured-by (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Formula")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (path (named (kind package) (name "Projections")) (named (kind allocation-def) (name "Map")) (anonymous (kind allocate) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Map")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Picture::detail")))
      (featured-by (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Picture")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Scene::inset")))
      (featured-by (node (document "memory://snapshot/sysml_owned_usage_projections.md") (qualified-name "Projections::Scene")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_owned_usage_projections.md") (range (start 5 17) (end 5 23)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/sysml_owned_usage_projections.md") (path (named (kind package) (name "Projections")) (named (kind allocation-def) (name "Map")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0) (authored-target "source")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_owned_usage_projections.md") (range (start 5 27) (end 5 33)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/sysml_owned_usage_projections.md") (path (named (kind package) (name "Projections")) (named (kind allocation-def) (name "Map")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0) (authored-target "target")
      (outcome (status unresolved)))
    )
  )
)
~~~
