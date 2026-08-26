# META
~~~ini
description=SysML 8.3.11.3 validatePartUsagePartDefinition requires at least one of the itemDefinitions of a PartUsage to be a PartDefinition
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.11.3 validatePartUsagePartDefinition
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.11.3:validatePartUsagePartDefinition
blocked_by=semantic-part-usage-without-part-definition
type=file
~~~
# SOURCE
~~~sysml
package Parts {
    part def Component;
    item def Material;
    part def Holder {
        // Conforming: the part usage is typed by a part definition.
        part good : Component;

        // Invalid: the part usage is typed only by an item definition.
        part bad : Material;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_part_usage_part_definition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "part_usage_without_part_definition")
        (source "semantic")
        (range (start 8 8) (end 8 28))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_part_usage_part_definition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:60ad8bbd1812006cc0fcd5553917e3c0ef2a77e11961710897edc6499d662ec1") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Material")))))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material"))) (kind item-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")))))
    (reference (id (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")))
      (subtype (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder")))
      (type (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder")))
      (type (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")))
      (subtype (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_part_usage_part_definition.md") (range (start 8 19) (end 8 27)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Material")))))
    )
  )
  (query (document "memory://snapshot/sysml_part_usage_part_definition.md") (range (start 5 20) (end 5 29)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_part_usage_part_definition.md") (qualified-name "Parts::Component")))))
    )
  )
)
~~~
