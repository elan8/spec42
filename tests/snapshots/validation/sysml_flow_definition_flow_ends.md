# META
~~~ini
description=SysML 8.3.16.2 validateFlowDefinitionFlowEnds forbids a FlowDefinition from having more than two flowEnds
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.16.2 validateFlowDefinitionFlowEnds
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.16.2:validateFlowDefinitionFlowEnds
type=file
~~~
# SOURCE
~~~sysml
package Flows {
    part def Component;

    // Conforming: a flow definition with two ends.
    flow def Binary {
        end source : Component;
        end target : Component;
    }

    // Invalid: a flow definition with three ends.
    flow def Ternary {
        end source : Component;
        end middle : Component;
        end target : Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_flow_definition_flow_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "invalid_binary_connection_like_end_count")
        (source "semantic")
        (range (start 10 4) (end 14 5))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_flow_definition_flow_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "invalid_binary_connection_like_end_count")
        (source "semantic")
        (range (start 10 4) (end 14 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a2fa93be61c86966513eab09b25c90e9e204292c649cf0e98fc2d27c829483a8") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary"))) (kind flow-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::source"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::target"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary"))) (kind flow-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::middle"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::source"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::target"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 2)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::middle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::source"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::target"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::middle"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::middle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::source"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::target"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::source"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::target"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::middle"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::source"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::target"))) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::source")))
      (featured-by (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary")))
      (type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::target")))
      (featured-by (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary")))
      (type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))
      (subtype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::source")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::target")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::middle")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::source")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary")))
      (positional-ends (authored 3) (effective 3))
    )
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::middle")))
      (featured-by (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary")))
      (type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::source")))
      (featured-by (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary")))
      (type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::target")))
      (featured-by (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary")))
      (type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (range (start 5 21) (end 5 30)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::source"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (range (start 6 21) (end 6 30)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Binary::target"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (range (start 12 21) (end 12 30)) (probe (position 12 21))
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::middle"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (range (start 11 21) (end 11 30)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::source"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (range (start 13 21) (end 13 30)) (probe (position 13 21))
    (reference (id (source (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Ternary::target"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_flow_definition_flow_ends.md") (qualified-name "Flows::Component")))))
    )
  )
)
~~~
