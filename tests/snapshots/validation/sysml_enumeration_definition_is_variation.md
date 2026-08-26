# META
~~~ini
description=SysML 8.3.8.2 validateEnumerationDefinitionIsVariation requires an EnumerationDefinition to be a variation
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.8.2 validateEnumerationDefinitionIsVariation
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.8.2:validateEnumerationDefinitionIsVariation
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the enum def keyword produces an EnumerationDefinition and makes it a variation at
// the same time, with each enum member as one variant.
//
// The violating side has no textual counterpart: SysML concrete syntax has no spelling for a
// non-variation EnumerationDefinition, so the rule is observable only as the accepted side
// pinned here.
package Enumerations {
    enum def Level {
        enum low;
        enum high;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_enumeration_definition_is_variation.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_enumeration_definition_is_variation.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8e7f3b13a559796098c3ee17654f13d7d9c2a3e4e1579c3cf203eb2487fea68d") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level::high"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level::low"))) (kind enum-literal) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level::high"))) (target (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level::low"))) (target (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level::high")))
      (featured-by (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level::low")))
      (featured-by (node (document "memory://snapshot/sysml_enumeration_definition_is_variation.md") (qualified-name "Enumerations::Level")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
