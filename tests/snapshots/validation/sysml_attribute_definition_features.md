# META
~~~ini
description=SysML 8.3.7.2 validateAttributeDefinitionFeatures requires all features of an AttributeDefinition to be non-composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.7.2 validateAttributeDefinitionFeatures
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.7.2:validateAttributeDefinitionFeatures
blocked_by=semantic-attribute-feature-composite
type=file
~~~
# SOURCE
~~~sysml
package Attributes {
    part def Component;

    // Conforming: every feature of the attribute definition is referential.
    attribute def Good {
        attribute reading;
    }

    // Invalid: a part usage member of an attribute definition is composite.
    attribute def Bad {
        part owned : Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_attribute_definition_features.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "attribute_feature_composite")
        (source "semantic")
        (range (start 10 8) (end 10 31))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_attribute_definition_features.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2aaac082e5b46db977296263c9b0566c0998796c5fc86dfe219a7ad5fdcc9984") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad::owned"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Good"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Good::reading"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad::owned"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad::owned"))) (target (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad::owned"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad::owned"))) (target (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Good::reading"))) (target (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad::owned")))
      (featured-by (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad")))
      (type (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Component")))
      (subtype (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad::owned")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Good::reading")))
      (featured-by (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_attribute_definition_features.md") (range (start 10 21) (end 10 30)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Bad::owned"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_definition_features.md") (qualified-name "Attributes::Component")))))
    )
  )
)
~~~
