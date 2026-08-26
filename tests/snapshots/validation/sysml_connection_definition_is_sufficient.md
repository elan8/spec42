# META
~~~ini
description=SysML 8.3.13.3 validateConnectionDefinitionIsSufficient requires a ConnectionDefinition to have isSufficient = true
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.13.3 validateConnectionDefinitionIsSufficient
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.13.3:validateConnectionDefinitionIsSufficient
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the connection def keyword produces a ConnectionDefinition and sets isSufficient
// at the same time.
//
// The violating side has no textual counterpart: SysML concrete syntax has no spelling that
// clears isSufficient on a ConnectionDefinition, so the rule is observable only as the accepted
// side pinned here.
package Connections {
    part def Component;
    connection def Link {
        end source : Component;
        end target : Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_connection_definition_is_sufficient.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_connection_definition_is_sufficient.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:17c4a96d149275e777a515462e7e5765226d0760993a95f2aaef4fbdfac7ea34") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::source"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::target"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::source"))) (target (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::target"))) (target (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::source"))) (target (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::target"))) (target (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")))
      (subtype (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::source")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::source")))
      (featured-by (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link")))
      (type (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::target")))
      (featured-by (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link")))
      (type (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (range (start 9 21) (end 9 30)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (range (start 10 21) (end 10 30)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_connection_definition_is_sufficient.md") (qualified-name "Connections::Component")))))
    )
  )
)
~~~
