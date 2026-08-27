# META
~~~ini
description=SysML 8.3.7.3 validateAttributeUsageFeatures requires all features of an AttributeUsage to be non-composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.7.3 validateAttributeUsageFeatures
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.7.3:validateAttributeUsageFeatures
blocked_by=semantic-attribute-feature-composite
type=file
~~~
# SOURCE
~~~sysml
package Attributes {
    part def Component;
    attribute def Reading;
    part def Holder {
        // Conforming: every feature of the attribute usage is referential.
        attribute good : Reading {
            attribute nested;
        }

        // Invalid: a part usage member of an attribute usage is composite.
        attribute bad : Reading {
            part owned : Component;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_attribute_usage_features.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "attribute_feature_composite")
        (source "semantic")
        (range (start 11 12) (end 11 35))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_attribute_usage_features.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2306cf84fc09da1b4bdb715192321cd247c9bb389f1fe7415972433b0b4ff636"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad::owned"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good::nested"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")))))
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad::owned"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Component")))))
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad"))) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad::owned"))) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad::owned"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good"))) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad"))) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad::owned"))) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good"))) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good::nested"))) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Component")))
      (subtype (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad::owned")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder")))
      (type (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad::owned")))
      (featured-by (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad")))
      (type (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Component")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Component")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder")))
      (type (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good::nested")))
      (featured-by (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")))
      (subtype (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_attribute_usage_features.md") (range (start 10 24) (end 10 31)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")))))
    )
  )
  (query (document "memory://snapshot/sysml_attribute_usage_features.md") (range (start 11 25) (end 11 34)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::bad::owned"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Component")))))
    )
  )
  (query (document "memory://snapshot/sysml_attribute_usage_features.md") (range (start 5 25) (end 5 32)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Holder::good"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_attribute_usage_features.md") (qualified-name "Attributes::Reading")))))
    )
  )
)
~~~
